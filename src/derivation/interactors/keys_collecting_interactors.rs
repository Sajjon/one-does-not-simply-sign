use crate::prelude::*;

/// A collection of "interactors" which can derive keys.
pub trait KeysCollectingInteractors {
    fn interactor_for(&self, kind: FactorSourceKind) -> KeyDerivationInteractor;
}

/// An interactor which can derive keys - either in parallel or serially.
pub enum KeyDerivationInteractor {
    Parallel(Arc<dyn DeriveKeyWithFactorParallelInteractor>),
    Serial(Arc<dyn DeriveKeyWithFactorSerialInteractor>),
}

impl KeyDerivationInteractor {
    pub fn parallel(interactor: Arc<dyn DeriveKeyWithFactorParallelInteractor>) -> Self {
        Self::Parallel(interactor)
    }

    pub fn serial(interactor: Arc<dyn DeriveKeyWithFactorSerialInteractor>) -> Self {
        Self::Serial(interactor)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelBatchKeyDerivationRequest {
    pub per_factor_source: IndexMap<FactorSourceIDFromHash, SerialBatchKeyDerivationRequest>,
}
impl ParallelBatchKeyDerivationRequest {
    pub fn new(
        per_factor_source: IndexMap<FactorSourceIDFromHash, SerialBatchKeyDerivationRequest>,
    ) -> Self {
        Self { per_factor_source }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialBatchKeyDerivationRequest {
    pub factor_source_id: FactorSourceIDFromHash,
    pub derivation_paths: IndexSet<DerivationPath>,
}
impl SerialBatchKeyDerivationRequest {
    pub fn new(
        factor_source_id: FactorSourceIDFromHash,
        derivation_paths: IndexSet<DerivationPath>,
    ) -> Self {
        Self {
            factor_source_id,
            derivation_paths,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchDerivationResponse {
    pub per_factor_source:
        IndexMap<FactorSourceIDFromHash, IndexSet<HierarchicalDeterministicFactorInstance>>,
}
impl BatchDerivationResponse {
    pub fn new(
        per_factor_source: IndexMap<
            FactorSourceIDFromHash,
            IndexSet<HierarchicalDeterministicFactorInstance>,
        >,
    ) -> Self {
        Self { per_factor_source }
    }
}

#[async_trait::async_trait]
pub trait DeriveKeyWithFactorParallelInteractor {
    async fn derive(
        &self,
        request: ParallelBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse>;
}

#[async_trait::async_trait]
pub trait DeriveKeyWithFactorSerialInteractor {
    async fn derive(
        &self,
        request: SerialBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse>;
}
