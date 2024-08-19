use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct Keyring {
    factor_source_id: FactorSourceID,
    paths: IndexSet<DerivationPath>,
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

    pub fn keyring_for(&self, factor_source_id: &FactorSourceID) -> Option<Keyring> {
        self.keyrings
            .borrow()
            .get(factor_source_id)
            .cloned()
            .inspect(|k| assert_eq!(k.factor_source_id, *factor_source_id))
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

    fn factor_and_path(factor_source: FactorSource, derivation_path: DerivationPath) -> Self {
        Self::new(IndexMap::from_iter([(
            factor_source.id,
            IndexSet::from_iter([derivation_path]),
        )]))
    }

    pub fn new_account_tx(
        factor_source: FactorSource,
        used_derivation_indices: impl UsedDerivationIndices,
    ) -> Self {
        let derivation_path =
            used_derivation_indices.next_derivation_path_account_tx(&factor_source);
        Self::factor_and_path(factor_source, derivation_path)
    }
}
