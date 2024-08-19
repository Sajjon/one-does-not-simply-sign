use crate::prelude::*;

/// A collection of "interactors" which can derive keys.
pub trait KeysCollectingInteractors {
    fn interactor_for(&self, kind: FactorSourceKind) -> KeyDerivationInteractor;
}

/// An interactor which can derove keys - either in parallel or serially.
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

pub struct ParallelBatchKeyDerivationRequest;
pub struct SerialBatchKeyDerivationRequest;
pub struct BatchDerivationResponse;

#[async_trait::async_trait]
pub trait DeriveKeyWithFactorParallelInteractor {
    async fn derive(
        &self,
        request: ParallelBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse>;
}

#[async_trait]
pub trait DeriveKeyWithFactorSerialInteractor {
    async fn derive(
        &self,
        request: SerialBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse>;
}
