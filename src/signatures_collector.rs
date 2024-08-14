use crate::prelude::*;

pub struct SignaturesCollectorState {
    petitions: RefCell<Petitions>,
}
impl SignaturesCollectorState {
    fn with_petitions(petitions: Petitions) -> Self {
        Self {
            petitions: RefCell::new(petitions),
        }
    }

    fn new(
        factor_to_txid: HashMap<FactorSourceID, IndexSet<IntentHash>>,
        txid_to_petition: IndexMap<IntentHash, PetitionOfTransaction>,
    ) -> Self {
        Self::with_petitions(Petitions::new(factor_to_txid, txid_to_petition))
    }
}

struct SignaturesCollectorDependencies {
    /// A collection of "interactors" used to sign with factor sources.
    interactors: Arc<dyn SignatureCollectingInteractors>,

    /// Factor sources grouped by kind, sorted according to "friction order",
    /// that is, we want to control which FactorSourceKind users sign with
    /// first, second etc, e.g. typically we prompt user to sign with Ledgers
    /// first, and if a user might lack access to that Ledger device, then it is
    /// best to "fail fast", otherwise we might waste the users time, if she has
    /// e.g. answered security questions and then is asked to use a Ledger
    /// she might not have handy at the moment - or might not be in front of a
    /// computer and thus unable to make a connection between the Radix Wallet
    /// and a Ledger device.
    factors_of_kind: IndexMap<FactorSourceKind, IndexSet<FactorSource>>,
}

impl SignaturesCollectorDependencies {
    fn new(
        interactors: Arc<dyn SignatureCollectingInteractors>,
        factors_of_kind: IndexMap<FactorSourceKind, IndexSet<FactorSource>>,
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
pub struct SignaturesCollector {
    dependencies: SignaturesCollectorDependencies,
    state: RefCell<SignaturesCollectorState>,
}

impl SignaturesCollector {
    pub fn new(
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        transactions: IndexSet<TransactionIntent>,
        signing_interactors_context: Arc<dyn SignatureCollectingInteractors>,
    ) -> Self {
        let mut petitions_for_all_transactions =
            IndexMap::<IntentHash, PetitionOfTransaction>::new();

        let all_factor_sources_in_profile = all_factor_sources_in_profile
            .into_iter()
            .map(|f| (f.id, f))
            .collect::<HashMap<FactorSourceID, FactorSource>>();

        let mut factor_to_payloads = HashMap::<FactorSourceID, IndexSet<IntentHash>>::new();

        let mut used_factor_sources = HashSet::<FactorSource>::new();

        let mut use_factor_in_tx = |id: &FactorSourceID, txid: &IntentHash| {
            if let Some(ref mut txids) = factor_to_payloads.get_mut(id) {
                txids.insert(txid.clone());
            } else {
                factor_to_payloads.insert(*id, IndexSet::from_iter([txid.clone()]));
            }

            assert!(!factor_to_payloads.is_empty());

            let factor_source = all_factor_sources_in_profile
                .get(id)
                .expect("Should have all factor sources");
            used_factor_sources.insert(factor_source.clone());

            assert!(!used_factor_sources.is_empty());
        };

        for transaction in transactions.into_iter() {
            let mut petitions_for_entities =
                HashMap::<AccountAddressOrIdentityAddress, BuilderEntity>::new();

            for entity in transaction.clone().entities_requiring_auth {
                let address = entity.address;
                match entity.security_state {
                    EntitySecurityState::Securified(sec) => {
                        let primary_role_matrix = sec;

                        let mut add = |factors: Vec<FactorInstance>| {
                            factors.into_iter().for_each(|f| {
                                let factor_source_id = f.factor_source_id;
                                use_factor_in_tx(&factor_source_id, &transaction.intent_hash);
                            })
                        };

                        add(primary_role_matrix.override_factors.clone());
                        add(primary_role_matrix.threshold_factors.clone());
                        let petition = BuilderEntity::new_securified(
                            transaction.intent_hash.clone(),
                            address.clone(),
                            primary_role_matrix,
                        );
                        petitions_for_entities.insert(address.clone(), petition);
                    }
                    EntitySecurityState::Unsecured(uec) => {
                        let factor_instance = uec;
                        let factor_source_id = factor_instance.factor_source_id;
                        use_factor_in_tx(&factor_source_id, &transaction.intent_hash);
                        let petition = BuilderEntity::new_unsecurified(
                            transaction.intent_hash.clone(),
                            address.clone(),
                            factor_instance,
                        );
                        petitions_for_entities.insert(address.clone(), petition);
                    }
                }
            }
            let petition_of_tx =
                PetitionOfTransaction::new(transaction.intent_hash.clone(), petitions_for_entities);

            petitions_for_all_transactions.insert(transaction.intent_hash, petition_of_tx);
        }

        let factors_of_kind = used_factor_sources
            .into_iter()
            .into_grouping_map_by(|x| x.kind())
            .collect::<IndexSet<FactorSource>>();

        let mut factors_of_kind = factors_of_kind
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().sorted().collect::<IndexSet<_>>()))
            .collect::<IndexMap<FactorSourceKind, IndexSet<FactorSource>>>();

        factors_of_kind.sort_keys();

        let state =
            SignaturesCollectorState::new(factor_to_payloads, petitions_for_all_transactions);

        Self {
            dependencies: SignaturesCollectorDependencies::new(
                signing_interactors_context,
                factors_of_kind,
            ),
            state: RefCell::new(state),
        }
    }
}

impl SignaturesCollector {
    /// If all transactions already would fail, or if all transactions already are done, then
    /// no point in continuing.
    ///
    /// `Ok(true)` means "continue", `Ok(false)` means "stop, we are done". `Err(_)` means "stop, we have failed".
    pub(super) fn continue_if_necessary(&self) -> Result<bool> {
        self.state
            .borrow()
            .petitions
            .borrow()
            .continue_if_necessary()
    }

    fn get_interactor(&self, kind: FactorSourceKind) -> SigningInteractor {
        self.dependencies.interactors.interactor_for(kind)
    }

    async fn use_certain_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
        kind: FactorSourceKind,
    ) -> Result<()> {
        assert!(factor_sources.iter().all(|f| f.kind() == kind));
        let interactor = self.get_interactor(kind);
        let client = SignWithFactorClient::new(interactor);
        let result = client
            .use_factor_sources(factor_sources.clone(), self)
            .await;
        match result {
            Ok(_) => {}
            Err(_) => self.process_batch_response(SignWithFactorSourceOrSourcesOutcome::Skipped {
                ids_of_skipped_factors_sources: factor_sources.into_iter().map(|f| f.id).collect(),
            }),
        }
        Ok(())
    }

    async fn use_factor_sources_in_decreasing_friction_order(&self) -> Result<()> {
        let factors_of_kind = self.dependencies.factors_of_kind.clone();
        for (kind, factor_sources) in factors_of_kind.into_iter() {
            self.use_certain_factor_sources(factor_sources, kind)
                .await?;

            if !self.continue_if_necessary()? {
                return Ok(()); // finished early, we have fulfilled signing requirements of all transactions
            }
        }
        Ok(())
    }
}

impl SignaturesCollector {
    fn input_for_parallel_batch_interactor(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> BatchTXBatchKeySigningRequest {
        self.state
            .borrow()
            .petitions
            .borrow()
            .input_for_parallel_batch_interactor(factor_source_id)
    }

    pub(super) fn request_for_serial_batch_interactor(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> SerialBatchSigningRequest {
        let batch_signing_request = self.input_for_parallel_batch_interactor(factor_source_id);

        SerialBatchSigningRequest::new(
            batch_signing_request,
            self.invalid_transactions_if_skipped(factor_source_id)
                .into_iter()
                .collect_vec(),
        )
    }

    pub(super) fn request_for_parallel_batch_interactor(
        &self,
        factor_source_ids: IndexSet<FactorSourceID>,
    ) -> SignWithFactorParallelInteractor {
        let per_factor_source = factor_source_ids
            .clone()
            .iter()
            .map(|fid| (*fid, self.input_for_parallel_batch_interactor(fid)))
            .collect::<IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>>();

        let invalid_transactions_if_skipped =
            self.invalid_transactions_if_skipped_factor_sources(factor_source_ids);

        // Prepare the request for the interactor
        SignWithFactorParallelInteractor::new(per_factor_source, invalid_transactions_if_skipped)
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

    pub(super) fn process_batch_response(
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
            .use_factor_sources_in_decreasing_friction_order()
            .await
            .inspect_err(|e| eprintln!("Failed to use factor sources: {:?}", e));
        self.state.into_inner().petitions.into_inner().outcome()
    }
}
