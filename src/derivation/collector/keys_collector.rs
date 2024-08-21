use crate::prelude::*;

/// A coordinator which gathers public keys from several factor sources of different
/// kinds, in increasing friction order, for many transactions and for
/// potentially multiple entities and for many factor instances (derivation paths)
/// for each transaction.
///
/// By increasing friction order we mean, the quickest and easiest to use FactorSourceKind
/// is last; namely `DeviceFactorSource`, and the most tedious FactorSourceKind is
/// first; namely `LedgerFactorSource`, which user might also lack access to.
pub struct KeysCollector {
    /// Stateless immutable values used by the collector to gather public keys
    /// from factor sources.
    dependencies: KeysCollectorDependencies,

    /// Mutable internal state of the collector which builds up the list
    /// of public keys from each used factor source.
    state: RefCell<KeysCollectorState>,
}

impl KeysCollector {
    fn with_preprocessor(
        all_factor_sources_in_profile: impl Into<IndexSet<FactorSource>>,
        interactors: Arc<dyn KeysCollectingInteractors>,
        preprocessor: KeysCollectorPreprocessor,
    ) -> Self {
        let all_factor_sources_in_profile = all_factor_sources_in_profile.into();
        let (keyrings, factors) = preprocessor.preprocess(all_factor_sources_in_profile);

        let dependencies = KeysCollectorDependencies::new(interactors, factors);
        let state = KeysCollectorState::new(keyrings);

        Self {
            dependencies,
            state: RefCell::new(state),
        }
    }

    pub fn new(
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        derivation_paths: IndexMap<FactorSourceID, IndexSet<DerivationPath>>,
        interactors: Arc<dyn KeysCollectingInteractors>,
    ) -> Self {
        let preprocessor = KeysCollectorPreprocessor::new(derivation_paths);
        Self::with_preprocessor(all_factor_sources_in_profile, interactors, preprocessor)
    }
}

impl KeysCollector {
    fn get_interactor(&self, kind: FactorSourceKind) -> KeyDerivationInteractor {
        self.dependencies.interactors.interactor_for(kind)
    }

    /// In decreasing "friction order"
    async fn derive_with_factors(&self) -> Result<()> {
        for factors_of_kind in self.dependencies.factors_of_kind.iter() {
            let interactor = self.get_interactor(factors_of_kind.kind);
            let client = KeysCollectingClient::new(interactor);
            client
                .use_factor_sources(factors_of_kind.factor_sources(), self)
                .await?;
        }
        Ok(())
    }
}

impl KeysCollector {
    fn input_for_interactor(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> SerialBatchKeyDerivationRequest {
        let keyring = self
            .state
            .borrow()
            .keyrings
            .borrow()
            .keyring_for(factor_source_id)
            .unwrap();
        assert_eq!(keyring.factors().len(), 0);
        let paths = keyring.paths.clone();
        SerialBatchKeyDerivationRequest::new(*factor_source_id, paths)
    }

    pub(crate) fn request_for_parallel_interactor(
        &self,
        factor_sources_ids: IndexSet<FactorSourceID>,
    ) -> ParallelBatchKeyDerivationRequest {
        ParallelBatchKeyDerivationRequest::new(
            factor_sources_ids
                .into_iter()
                .map(|f| (f, self.input_for_interactor(&f)))
                .collect(),
        )
    }

    pub(crate) fn request_for_serial_interactor(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> SerialBatchKeyDerivationRequest {
        self.input_for_interactor(factor_source_id)
    }

    pub(crate) fn process_batch_response(&self, response: BatchDerivationResponse) {
        self.state.borrow_mut().process_batch_response(response)
    }
}

impl KeysCollector {
    pub async fn collect_keys(self) -> KeyDerivationOutcome {
        _ = self
            .derive_with_factors() // in decreasing "friction order"
            .await
            .inspect_err(|e| eprintln!("Failed to use factor sources: {:?}", e));
        self.state.into_inner().keyrings.into_inner().outcome()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyDerivationOutcome {
    pub factors_by_source: IndexMap<FactorSourceID, IndexSet<FactorInstance>>,
}
impl KeyDerivationOutcome {
    pub fn new(factors_by_source: IndexMap<FactorSourceID, IndexSet<FactorInstance>>) -> Self {
        Self { factors_by_source }
    }

    /// ALL factor instances derived by the KeysCollector
    pub fn all_factors(&self) -> IndexSet<FactorInstance> {
        self.factors_by_source
            .clone()
            .into_iter()
            .flat_map(|(_, v)| v)
            .collect()
    }
}
