use crate::prelude::*;

pub struct FactorOutputCollectorDependencies<ParallelInteractor, SerialInteractor> {
    /// A collection of "interactors" used to sign with factor sources.
    pub interactors: Arc<dyn InteractorsContext<ParallelInteractor, SerialInteractor>>,

    /// Factor sources grouped by kind, sorted according to "friction order",
    /// that is, we want to control which FactorSourceKind users sign with
    /// first, second etc, e.g. typically we prompt user to sign with Ledgers
    /// first, and if a user might lack access to that Ledger device, then it is
    /// best to "fail fast", otherwise we might waste the users time, if she has
    /// e.g. answered security questions and then is asked to use a Ledger
    /// she might not have handy at the moment - or might not be in front of a
    /// computer and thus unable to make a connection between the Radix Wallet
    /// and a Ledger device.
    pub factors_of_kind: IndexSet<FactorSourcesOfKind>,
}

pub type SignaturesCollectorDependencies = FactorOutputCollectorDependencies<
    SignWithFactorParallelInteractor,
    SignWithFactorSerialInteractor,
>;

impl SignaturesCollectorDependencies {
    pub fn new(
        interactors: Arc<SignatureCollectingInteractors>,
        factors_of_kind: IndexSet<FactorSourcesOfKind>,
    ) -> Self {
        Self {
            interactors,
            factors_of_kind,
        }
    }
}

/// A coordinator which gathers signatures from several factor sources of different
/// kinds, in increasing friction order, for many transactions and for
/// potentially multiple entities and for many factor instances (derivation paths)
/// for each transaction.
///
/// By increasing friction order we mean, the quickest and easiest to use FactorSourceKind
/// is last; namely `DeviceFactorSource`, and the most tedious FactorSourceKind is
/// first; namely `LedgerFactorSource`, which user might also lack access to.
pub type SignaturesCollector = FactorOutputCollector<
    SignaturesCollectorState,
    SignWithFactorParallelInteractor,
    SignWithFactorSerialInteractor,
>;

pub trait IsFactorOutputCollectorState {
    fn continue_if_necessary(&self) -> Result<bool>;
}

pub struct FactorOutputCollector<State, ParallelInteractor, SerialInteractor>
where
    State: IsFactorOutputCollectorState,
{
    /// Stateless immutable values used by the collector to gather signatures
    /// from factor sources.
    dependencies: FactorOutputCollectorDependencies<ParallelInteractor, SerialInteractor>,

    /// Mutable internal state of the collector which builds up the list
    /// of signatures from each used factor source.
    state: RefCell<State>,
}

impl<State, ParallelInteractor, SerialInteractor>
    FactorOutputCollector<State, ParallelInteractor, SerialInteractor>
where
    State: IsFactorOutputCollectorState,
{
    fn with(
        dependencies: FactorOutputCollectorDependencies<ParallelInteractor, SerialInteractor>,
        state: State,
    ) -> Self {
        Self {
            dependencies,
            state: RefCell::new(state),
        }
    }

    /// If all transactions already would fail, or if all transactions already are done, then
    /// no point in continuing.
    ///
    /// `Ok(true)` means "continue", `Ok(false)` means "stop, we are done". `Err(_)` means "stop, we have failed".
    pub(crate) fn continue_if_necessary(&self) -> Result<bool> {
        self.state.borrow().continue_if_necessary()
    }

    fn get_interactor(
        &self,
        kind: FactorSourceKind,
    ) -> UseFactorInteractor<ParallelInteractor, SerialInteractor> {
        self.dependencies.interactors.interactor_for(kind)
    }
}

impl SignaturesCollector {
    pub fn new(
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        transactions: IndexSet<TransactionIntent>,
        interactors: Arc<SignatureCollectingInteractors>,
    ) -> Self {
        let preprocessor = SignaturesCollectorPreprocessor::new(transactions);
        let (petitions, factors) = preprocessor.preprocess(all_factor_sources_in_profile);

        let dependencies = SignaturesCollectorDependencies::new(interactors, factors);
        let state = SignaturesCollectorState::new(petitions);

        Self::with(dependencies, state)
    }
}

impl SignaturesCollector {
    async fn use_interactor(
        &self,
        interactor: SigningInteractor,
        factor_sources: IndexSet<FactorSource>,
    ) -> Result<()> {
        match &interactor {
            // Parallel Interactor: Many Factor Sources at once
            SigningInteractor::Parallel(interactor) => {
                // Prepare the request for the interactor
                let request = self.request_for_parallel_interactor(
                    factor_sources.into_iter().map(|f| f.id).collect(),
                );
                let response = interactor.sign(request).await?;
                self.process_batch_response(response);
            }

            // Serial Interactor: One Factor Sources at a time
            // After each factor source we pass the result to the collector
            // updating its internal state so that we state about being able
            // to skip the next factor source or not.
            SigningInteractor::Serial(interactor) => {
                for factor_source in factor_sources {
                    // Prepare the request for the interactor
                    let request = self.request_for_serial_interactor(&factor_source.id);

                    // Produce the results from the interactor
                    let response = interactor.sign(request).await?;

                    // Report the results back to the collector
                    self.process_batch_response(response);

                    if !self.continue_if_necessary()? {
                        break;
                    }
                }
            }
        }
        Ok(())
    }

    async fn sign_with_factors_of_kind(
        &self,
        factor_sources_of_kind: FactorSourcesOfKind,
    ) -> Result<()> {
        let interactor = self.get_interactor(factor_sources_of_kind.kind);
        let result = self
            .use_interactor(interactor, factor_sources_of_kind.factor_sources())
            .await;
        match result {
            Ok(_) => {}
            Err(_) => self.process_batch_response(SignWithFactorSourceOrSourcesOutcome::Skipped {
                ids_of_skipped_factors_sources: factor_sources_of_kind.factor_source_ids(),
            }),
        }
        Ok(())
    }

    /// In decreasing "friction order"
    async fn sign_with_factors(&self) -> Result<()> {
        let factors_of_kind = self.dependencies.factors_of_kind.clone();
        for factor_sources_of_kind in factors_of_kind.into_iter() {
            self.sign_with_factors_of_kind(factor_sources_of_kind)
                .await?;

            if !self.continue_if_necessary()? {
                break; // finished early, we have fulfilled signing requirements of all transactions
            }
        }
        Ok(())
    }
}

impl SignaturesCollector {
    fn input_for_interactor(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> BatchTXBatchKeySigningRequest {
        self.state
            .borrow()
            .petitions
            .borrow()
            .input_for_interactor(factor_source_id)
    }

    pub(crate) fn request_for_serial_interactor(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> SerialBatchSigningRequest {
        let batch_signing_request = self.input_for_interactor(factor_source_id);

        SerialBatchSigningRequest::new(
            batch_signing_request,
            self.invalid_transactions_if_skipped(factor_source_id)
                .into_iter()
                .collect_vec(),
        )
    }

    pub(crate) fn request_for_parallel_interactor(
        &self,
        factor_source_ids: IndexSet<FactorSourceID>,
    ) -> ParallelBatchSigningRequest {
        let per_factor_source = factor_source_ids
            .clone()
            .iter()
            .map(|fid| (*fid, self.input_for_interactor(fid)))
            .collect::<IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>>();

        let invalid_transactions_if_skipped =
            self.invalid_transactions_if_skipped_factor_sources(factor_source_ids);

        // Prepare the request for the interactor
        ParallelBatchSigningRequest::new(per_factor_source, invalid_transactions_if_skipped)
    }

    pub(super) fn invalid_transactions_if_skipped(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        self.state
            .borrow()
            .petitions
            .borrow()
            .invalid_transactions_if_skipped(factor_source_id)
    }

    fn invalid_transactions_if_skipped_factor_sources(
        &self,
        factor_source_ids: IndexSet<FactorSourceID>,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        factor_source_ids
            .into_iter()
            .flat_map(|f| self.invalid_transactions_if_skipped(&f))
            .collect::<IndexSet<_>>()
    }

    pub(crate) fn process_batch_response(
        &self,
        response: SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>,
    ) {
        let state = self.state.borrow_mut();
        let petitions = state.petitions.borrow_mut();
        petitions.process_batch_response(response)
    }
}

impl SignaturesCollector {
    pub async fn collect_signatures(self) -> SignaturesOutcome {
        _ = self
            .sign_with_factors() // in decreasing "friction order"
            .await
            .inspect_err(|e| eprintln!("Failed to use factor sources: {:?}", e));
        self.state.into_inner().petitions.into_inner().outcome()
    }
}
