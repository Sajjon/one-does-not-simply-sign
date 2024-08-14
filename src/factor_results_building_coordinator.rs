use crate::prelude::*;

/// A coordinator which gathers signatures from several factor sources of different
/// kinds, in increasing friction order, for many transactions and for
/// potentially multiple entities and for many factor instances (derivation paths)
/// for each transaction.
///
/// By increasing friction order we mean, the quickest and easiest to use FactorSourceKind
/// is last; namely `DeviceFactorSource`, and the most tedious FactorSourceKind is
/// first; namely `LedgerFactorSource`, which user might also lack access to.
pub struct FactorResultsBuildingCoordinator {
    /// A context of drivers for "using" factor sources - either to (batch) sign
    /// transaction(s) with, or to derive public keys from.
    drivers: Arc<dyn IsUseFactorSourceDriversContext>,

    /// Factor sources grouped by kind, sorted according to "friction order",
    /// that is, we want to control which FactorSourceKind users use
    /// first, second etc, e.g. typically we prompt user to use Ledgers
    /// first, and if a user might lack access to that Ledger device, then it is
    /// best to "fail fast", otherwise we might waste the users time, if she has
    /// e.g. answered security questions and then is asked to use a Ledger
    /// she might not have handy at the moment - or might not be in front of a
    /// computer and thus unable to make a connection between the Radix Wallet
    /// and a Ledger device.
    factors_of_kind: IndexMap<FactorSourceKind, IndexSet<FactorSource>>,

    /// FactorSource output builders for each factor source, for each OutputGroup
    /// (Transaction or SecurityStructureOfFactorInstances), for each entity to
    /// produce an output (signature or public key).
    builders: RefCell<Builders>,
}

impl FactorResultsBuildingCoordinator {
    pub fn new(
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        transactions: IndexSet<TransactionIntent>,
        signing_drivers_context: Arc<dyn IsUseFactorSourceDriversContext>,
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

        let petitions = Builders::new(factor_to_payloads, petitions_for_all_transactions);

        Self {
            drivers: signing_drivers_context,
            factors_of_kind,
            builders: petitions.into(),
        }
    }
}

impl FactorResultsBuildingCoordinator {
    /// If all transactions already would fail, or if all transactions already are done, then
    /// no point in continuing.
    ///
    /// `Ok(true)` means "continue", `Ok(false)` means "stop, we are done". `Err(_)` means "stop, we have failed".
    pub(crate) fn continue_if_necessary(&self) -> Result<bool> {
        self.builders.borrow().continue_if_necessary()
    }

    fn get_driver(&self, kind: FactorSourceKind) -> UseFactorSourceDriver {
        self.drivers.driver_for_factor_source_kind(kind)
    }

    async fn use_certain_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
        kind: FactorSourceKind,
    ) -> Result<()> {
        assert!(factor_sources.iter().all(|f| f.kind() == kind));
        let driver = self.get_driver(kind);
        let client = UseFactorSourceClient::new(driver);
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
        let factors_of_kind = self.factors_of_kind.clone();
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

impl FactorResultsBuildingCoordinator {
    pub(crate) fn requests_for_serial_single_driver(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> IndexMap<IntentHash, IndexSet<SerialSingleSigningRequestFull>> {
        let invalid_transactions_if_skipped =
            self.invalid_transactions_if_skipped(factor_source_id);

        self.builders
            .borrow()
            .inputs_for_serial_single_driver(factor_source_id)
            .into_iter()
            .map(|(intent_hash, requests)| {
                let values = requests
                    .into_iter()
                    .map(|p| {
                        SerialSingleSigningRequestFull::new(
                            p,
                            invalid_transactions_if_skipped.clone(),
                        )
                    })
                    .collect::<IndexSet<_>>();
                (intent_hash, values)
            })
            .collect()
    }

    fn input_for_parallel_batch_driver(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> BatchTXBatchKeySigningRequest {
        self.builders
            .borrow()
            .input_for_parallel_batch_driver(factor_source_id)
    }

    pub(crate) fn request_for_serial_batch_driver(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> SerialBatchSigningRequest {
        let batch_signing_request = self.input_for_parallel_batch_driver(factor_source_id);

        SerialBatchSigningRequest::new(
            batch_signing_request,
            self.invalid_transactions_if_skipped(factor_source_id)
                .into_iter()
                .collect_vec(),
        )
    }

    pub(crate) fn request_for_parallel_batch_driver(
        &self,
        factor_source_ids: IndexSet<FactorSourceID>,
    ) -> ParallelBatchSigningRequest {
        let per_factor_source = factor_source_ids
            .clone()
            .iter()
            .map(|fid| (*fid, self.input_for_parallel_batch_driver(fid)))
            .collect::<IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>>();

        let invalid_transactions_if_skipped =
            self.invalid_transactions_if_skipped_factor_sources(factor_source_ids);

        // Prepare the request for the driver
        ParallelBatchSigningRequest::new(per_factor_source, invalid_transactions_if_skipped)
    }

    pub fn invalid_transactions_if_skipped(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        self.builders
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
        let petitions = self.builders.borrow_mut();
        petitions.process_batch_response(response)
    }
}

impl FactorResultsBuildingCoordinator {
    pub async fn use_factor_sources(self) -> SignaturesOutcome {
        _ = self
            .use_factor_sources_in_decreasing_friction_order()
            .await
            .inspect_err(|e| eprintln!("Failed to use factor sources: {:?}", e));
        self.builders.into_inner().outcome()
    }
}
