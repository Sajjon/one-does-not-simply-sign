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

pub enum KeySpace {
    Unsecurified,
    Securified,
}

pub trait UsedDerivationIndices {
    fn next_derivation_index_for(
        &self,
        factor_source: &FactorSource,
        key_kind: KeyKind,
        entity_kind: EntityKind,
        key_space: KeySpace,
    ) -> DerivationIndex;

    fn next_derivation_path(
        &self,
        factor_source: &FactorSource,
        key_kind: KeyKind,
        entity_kind: EntityKind,
        key_space: KeySpace,
    ) -> DerivationPath {
        let index = self.next_derivation_index_for(factor_source, key_kind, entity_kind, key_space);
        DerivationPath::new(entity_kind, key_kind, index)
    }

    fn next_derivation_path_account_tx(&self, factor_source: &FactorSource) -> DerivationPath {
        self.next_derivation_path(
            factor_source,
            KeyKind::T9n,
            EntityKind::Account,
            KeySpace::Unsecurified,
        )
    }
}

pub struct DefaultUsedDerivationIndices {
    keys: IndexMap<FactorSourceID, Keyrings>,
}

impl UsedDerivationIndices for DefaultUsedDerivationIndices {
    fn next_derivation_index_for(
        &self,
        factor_source: &FactorSource,
        key_kind: KeyKind,
        entity_kind: EntityKind,
        key_space: KeySpace,
    ) -> DerivationIndex {
        todo!()
    }
}

impl KeysCollector {
    pub fn new(
        all_factor_sources_in_profile: IndexSet<FactorSource>,
        // derivation_paths: IndexMap<FactorSourceID, IndexSet<DerivationPath>>,
        interactors: Arc<dyn KeysCollectingInteractors>,
        preprocessor: KeysCollectorPreprocessor,
    ) -> Self {
        let (keyrings, factors) = preprocessor.preprocess(all_factor_sources_in_profile);

        let dependencies = KeysCollectorDependencies::new(interactors, factors);
        let state = KeysCollectorState::new(keyrings);

        Self {
            dependencies,
            state: RefCell::new(state),
        }
    }

    pub fn new_account_tx(
        factor_source: FactorSource,
        used_derivation_indices: impl UsedDerivationIndices,
    ) -> Self {
        todo!()
    }
}

impl KeysCollector {
    pub async fn collect_keys(self) -> KeyDerivationOutcome {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KeyDerivationOutcome {
    pub keys: IndexSet<FactorInstance>,
}
