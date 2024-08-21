use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Keyring {
    pub factor_source_id: FactorSourceID,
    pub paths: IndexSet<DerivationPath>,
    derived: RefCell<IndexSet<FactorInstance>>,
}

impl Keyring {
    pub fn new(factor_source_id: FactorSourceID, paths: IndexSet<DerivationPath>) -> Self {
        Self {
            factor_source_id,
            paths,
            derived: RefCell::new(IndexSet::new()),
        }
    }
    pub fn factors(&self) -> IndexSet<FactorInstance> {
        self.derived.borrow().clone()
    }

    pub(crate) fn process_response(&self, response: IndexSet<FactorInstance>) {
        assert!(response
            .iter()
            .all(|f| f.factor_source_id == self.factor_source_id
                && !self
                    .derived
                    .borrow()
                    .iter()
                    .any(|x| x.hd_public_key == f.hd_public_key)));

        self.derived.borrow_mut().extend(response)
    }
}

#[derive(Default, Clone, Debug)]
pub struct Keyrings {
    keyrings: RefCell<IndexMap<FactorSourceID, Keyring>>,
}

impl Keyrings {
    pub fn new(derivation_paths: IndexMap<FactorSourceID, IndexSet<DerivationPath>>) -> Self {
        let keyrings = derivation_paths
            .into_iter()
            .map(|(factor_source_id, derivation_paths)| {
                (
                    factor_source_id,
                    Keyring::new(factor_source_id, derivation_paths),
                )
            })
            .collect::<IndexMap<FactorSourceID, Keyring>>();
        Self {
            keyrings: RefCell::new(keyrings),
        }
    }

    pub fn outcome(self) -> KeyDerivationOutcome {
        let key_rings = self.keyrings.into_inner();
        KeyDerivationOutcome::new(
            key_rings
                .into_iter()
                .map(|(k, v)| (k, v.factors()))
                .collect(),
        )
    }

    pub fn keyring_for(&self, factor_source_id: &FactorSourceID) -> Option<Keyring> {
        self.keyrings
            .borrow()
            .get(factor_source_id)
            .cloned()
            .inspect(|k| assert_eq!(k.factor_source_id, *factor_source_id))
    }

    pub(crate) fn process_batch_response(&self, response: BatchDerivationResponse) {
        for (factor_source_id, factors) in response.per_factor_source.into_iter() {
            let mut rings = self.keyrings.borrow_mut();
            let keyring = rings.get_mut(&factor_source_id).unwrap();
            keyring.process_response(factors)
        }
    }
}

pub struct KeysCollectorPreprocessor {
    derivation_paths: IndexMap<FactorSourceID, IndexSet<DerivationPath>>,
}

impl KeysCollectorPreprocessor {
    pub fn new(derivation_paths: IndexMap<FactorSourceID, IndexSet<DerivationPath>>) -> Self {
        Self { derivation_paths }
    }

    pub(crate) fn preprocess(
        &self,
        all_factor_sources_in_profile: IndexSet<FactorSource>,
    ) -> (Keyrings, IndexSet<FactorSourcesOfKind>) {
        let all_factor_sources_in_profile = all_factor_sources_in_profile
            .into_iter()
            .map(|f| (f.id, f))
            .collect::<HashMap<FactorSourceID, FactorSource>>();

        let factor_sources_of_kind = sort_group_factors(
            self.derivation_paths
                .clone()
                .keys()
                .map(|id| {
                    all_factor_sources_in_profile
                        .get(id)
                        .expect("Should have all factor sources")
                        .clone()
                })
                .collect::<HashSet<_>>(),
        );
        let keyrings = Keyrings::new(self.derivation_paths.clone());
        (keyrings, factor_sources_of_kind)
    }
}
