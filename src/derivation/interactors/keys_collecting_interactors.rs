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
    per_factor_source: IndexMap<FactorSourceID, SerialBatchKeyDerivationRequest>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialBatchKeyDerivationRequest {
    factor_source_id: FactorSourceID,
    derivation_paths: IndexSet<DerivationPath>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchDerivationResponse {
    per_factor_source: IndexMap<FactorSourceID, IndexSet<FactorInstance>>,
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
