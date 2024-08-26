use crate::prelude::*;

pub struct SignaturesCollectorPreprocessor {
    transactions: IndexSet<TXToSign>,
}

pub fn sort_group_factors(
    used_factor_sources: HashSet<HDFactorSource>,
) -> IndexSet<FactorSourcesOfKind> {
    let factors_of_kind = used_factor_sources
        .into_iter()
        .into_grouping_map_by(|x| x.factor_source_kind())
        .collect::<IndexSet<HDFactorSource>>();

    let mut factors_of_kind = factors_of_kind
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().sorted().collect::<IndexSet<_>>()))
        .collect::<IndexMap<FactorSourceKind, IndexSet<HDFactorSource>>>();

    factors_of_kind.sort_keys();

    factors_of_kind
        .into_iter()
        .map(|(k, v)| FactorSourcesOfKind::new(k, v).unwrap())
        .collect::<IndexSet<_>>()
}

impl SignaturesCollectorPreprocessor {
    pub(super) fn new(transactions: IndexSet<TXToSign>) -> Self {
        Self { transactions }
    }

    pub(super) fn preprocess(
        self,
        all_factor_sources_in_profile: IndexSet<HDFactorSource>,
    ) -> (Petitions, IndexSet<FactorSourcesOfKind>) {
        let transactions = self.transactions;
        let mut petitions_for_all_transactions = IndexMap::<IntentHash, PetitionTransaction>::new();

        let all_factor_sources_in_profile = all_factor_sources_in_profile
            .into_iter()
            .map(|f| (f.factor_source_id(), f))
            .collect::<HashMap<FactorSourceIDFromHash, HDFactorSource>>();

        let mut factor_to_payloads = HashMap::<FactorSourceIDFromHash, IndexSet<IntentHash>>::new();

        let mut used_factor_sources = HashSet::<HDFactorSource>::new();

        let mut use_factor_in_tx = |id: &FactorSourceIDFromHash, txid: &IntentHash| {
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
                HashMap::<AddressOfAccountOrPersona, PetitionEntity>::new();

            for entity in transaction.entities_requiring_auth() {
                let address = entity.address();
                match entity.security_state() {
                    EntitySecurityState::Securified(sec) => {
                        let primary_role_matrix = sec;

                        let mut add = |factors: Vec<HierarchicalDeterministicFactorInstance>| {
                            factors.into_iter().for_each(|f| {
                                let factor_source_id = f.factor_source_id;
                                use_factor_in_tx(&factor_source_id, &transaction.intent_hash);
                            })
                        };

                        add(primary_role_matrix.override_factors.clone());
                        add(primary_role_matrix.threshold_factors.clone());
                        let petition = PetitionEntity::new_securified(
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
                        let petition = PetitionEntity::new_unsecurified(
                            transaction.intent_hash.clone(),
                            address.clone(),
                            factor_instance,
                        );
                        petitions_for_entities.insert(address.clone(), petition);
                    }
                }
            }

            let petition_of_tx =
                PetitionTransaction::new(transaction.intent_hash.clone(), petitions_for_entities);

            petitions_for_all_transactions.insert(transaction.intent_hash, petition_of_tx);
        }

        let factors_of_kind = sort_group_factors(used_factor_sources);

        let petitions = Petitions::new(factor_to_payloads, petitions_for_all_transactions);

        (petitions, factors_of_kind)
    }
}
