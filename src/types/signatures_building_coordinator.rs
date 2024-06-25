use std::io::Write;

use crate::prelude::*;

/// A coordinator which gathers signatures from several factor sources of different
/// kinds for many transactions and for potentially multiple derivation paths
/// for each transaction.
pub struct SignaturesBuildingCoordinator {
    /// A context of drivers used to sign with factor sources.
    signing_drivers_context: Arc<dyn IsSigningDriversContext>,

    /// Factor sources grouped by kind, sorted according to "signing order",
    /// that is, we want to control which factor source kind users signs with
    /// first, second etc, e.g. typically we prompt user to sign with Ledgers
    /// first, and if a user might lack access to that Ledger device, then it is
    /// best to "fail fast", otherwise we might waste the users time, if she has
    /// e.g. answered security questions and then is asked to sign with a Ledger
    /// she might not have handy at the moment - or might not be in front of a
    /// computer and thus unable to make a connection between the Radix Wallet
    /// and a Ledger device.
    factors_of_kind: IndexMap<FactorSourceKind, IndexSet<FactorSource>>,

    /// Signature builders for each factor source, for each transaction,
    /// for each entity to sign.
    petitions: RefCell<Petitions>,
}

impl SignaturesBuildingCoordinator {
    pub fn new(
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        transactions: IndexSet<TransactionIntent>,
        signing_drivers_context: Arc<dyn IsSigningDriversContext>,
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
                factor_to_payloads.insert(id.clone(), IndexSet::from_iter([txid.clone()]));
            }

            assert!(!factor_to_payloads.is_empty());

            let factor_source = all_factor_sources_in_profile
                .get(id)
                .expect("Should have all factor sources");
            used_factor_sources.insert(factor_source.clone());

            assert!(!used_factor_sources.is_empty());
        };

        for (index, transaction) in transactions.into_iter().enumerate() {
            let transaction_index = TransactionIndex::new(index, transaction.intent_hash.clone());

            let mut petitions_for_entities =
                HashMap::<AccountAddressOrIdentityAddress, PetitionOfTransactionByEntity>::new();

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

                        // let builder = SignaturesBuilderLevel2::new_securified(
                        //     address.clone(),
                        //     primary_role_matrix,
                        // );
                        // builders_level_2.insert(address.clone(), builder);
                        let petition = PetitionOfTransactionByEntity::new_securified(
                            transaction_index.clone(),
                            address.clone(),
                            primary_role_matrix,
                        );
                        petitions_for_entities.insert(address.clone(), petition);
                    }
                    EntitySecurityState::Unsecured(uec) => {
                        let factor_instance = uec;
                        let factor_source_id = factor_instance.factor_source_id;
                        use_factor_in_tx(&factor_source_id, &transaction.intent_hash);

                        // let builder =
                        //     SignaturesBuilderLevel2::new_unsecurified(
                        //         address.clone(),
                        //         factor_instance,
                        //     );
                        // builders_level_2
                        //     .insert(address.clone(), builder);

                        let petition = PetitionOfTransactionByEntity::new_unsecurified(
                            transaction_index.clone(),
                            address.clone(),
                            factor_instance,
                        );
                        petitions_for_entities.insert(address.clone(), petition);
                    }
                }
            }
            // builders_level_0.insert(
            //     transaction.intent_hash.clone(),
            //     SignaturesBuilderLevel1::new(
            //         transaction.intent_hash.clone(),
            //         builders_level_2,
            //     ),
            // );

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

        let petitions = Petitions::new(factor_to_payloads, petitions_for_all_transactions);

        Self {
            signing_drivers_context,
            factors_of_kind,
            petitions: petitions.into(),
        }
    }
}

impl SignaturesBuildingCoordinator {
    /// If all transactions already would fail, or if all transactions already are done, then
    /// no point in continuing.
    ///
    /// `Ok(true)` means "continue", `Ok(false)` means "stop, we are done". `Err(_)` means "stop, we have failed".
    fn continue_if_necessary(&self) -> Result<bool> {
        self.petitions.borrow().continue_if_necessary()
    }

    fn get_driver(&self, kind: FactorSourceKind) -> SigningDriver {
        self.signing_drivers_context
            .driver_for_factor_source_kind(kind)
    }

    async fn sign_with_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
        kind: FactorSourceKind,
    ) -> Result<()> {
        assert!(factor_sources.iter().all(|f| f.kind() == kind));
        let signing_driver = self.get_driver(kind);
        signing_driver.sign(factor_sources, self).await;
        Ok(())
    }

    async fn do_sign(&self) -> Result<()> {
        let factors_of_kind = self.factors_of_kind.clone();
        for (kind, factor_sources) in factors_of_kind.into_iter() {
            println!("✍🏻 sign with factor sources: {:?}", factor_sources);
            self.sign_with_factor_sources(factor_sources, kind).await?;
            let should_continue = self.continue_if_necessary()?;
            if !should_continue {
                println!("🌈 finished early");
                return Ok(()); // finished early, we have fulfilled signing requirements of all transactions
            }
        }
        Ok(())
    }
}

impl SignaturesBuildingCoordinator {
    pub(crate) fn inputs_for_serial_single_driver(
        &self,
        factor_source: FactorSource,
    ) -> IndexMap<IntentHash, IndexSet<SerialSingleSigningRequest>> {
        self.petitions
            .borrow()
            .inputs_for_serial_single_driver(factor_source)
    }

    pub(crate) fn input_for_parallel_batch_driver(
        &self,
        factor_source: FactorSource,
    ) -> BatchTXBatchKeySigningRequest {
        self.petitions
            .borrow()
            .input_for_parallel_batch_driver(factor_source)
    }

    pub fn invalid_transactions_if_skipped(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        self.petitions
            .borrow()
            .invalid_transactions_if_skipped(factor_source_id)
    }

    pub fn invalid_transactions_if_skipped_factor_sources(
        &self,
        factor_source_ids: IndexSet<FactorSourceID>,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        factor_source_ids
            .into_iter()
            .flat_map(|f| self.invalid_transactions_if_skipped(&f))
            .collect::<IndexSet<_>>()
    }

    /// Returns `true` if we should continue, `false` if we should stop.
    pub(crate) fn process_single_response(
        &self,
        response: SignWithFactorSourceOrSourcesOutcome<HDSignature>,
    ) -> bool {
        println!("🔥 process_single_response: {:?}", response);
        {
            let petitions = self.petitions.borrow_mut();
            petitions.process_single_response(response);
        }
        self.continue_if_necessary().unwrap()
    }
    pub(crate) fn process_batch_response(
        &self,
        response: SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>,
    ) {
        println!("🔥 process_batch_response: {:?}", response);
        let petitions = self.petitions.borrow_mut();
        petitions.process_batch_response(response)
    }
}

impl SignaturesBuildingCoordinator {
    pub async fn sign(self) -> Result<SignaturesOutcome> {
        self.do_sign().await?;
        let outcome = self.petitions.into_inner().outcome();
        Ok(outcome)
    }
}
