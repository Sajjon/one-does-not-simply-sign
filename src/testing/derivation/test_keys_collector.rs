use crate::prelude::*;

pub struct TestDerivationInteractors;

impl KeysCollectingInteractors for TestDerivationInteractors {
    fn interactor_for(&self, kind: FactorSourceKind) -> KeyDerivationInteractor {
        match kind {
            FactorSourceKind::Device => {
                KeyDerivationInteractor::parallel(Arc::new(TestDerivationParallelInteractor))
            }
            _ => KeyDerivationInteractor::serial(Arc::new(TestDerivationSerialInteractor)),
        }
    }
}

pub struct TestDerivationParallelInteractor;

#[async_trait::async_trait]
impl DeriveKeyWithFactorParallelInteractor for TestDerivationParallelInteractor {
    async fn derive(
        &self,
        request: ParallelBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse> {
        Err(CommonError::Failure)
    }
}

pub struct TestDerivationSerialInteractor;

#[async_trait::async_trait]
impl DeriveKeyWithFactorSerialInteractor for TestDerivationSerialInteractor {
    async fn derive(
        &self,
        request: SerialBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse> {
        Err(CommonError::Failure)
    }
}

impl KeysCollector {
    pub fn new_test(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        derivation_paths: impl IntoIterator<Item = (FactorSourceID, IndexSet<DerivationPath>)>,
    ) -> Self {
        Self::new(
            all_factor_sources_in_profile.into_iter().collect(),
            derivation_paths.into_iter().collect(),
            Arc::new(TestDerivationInteractors),
        )
    }

    /// mainnet
    pub fn new_account_tx(factor_source: FactorSource) -> Self {
        let indices = DefaultUsedDerivationIndices::default();
        let path = indices.next_derivation_path_account_tx(factor_source.id, NetworkID::Mainnet);
        Self::new_test(
            [factor_source.clone()],
            [(factor_source.id, IndexSet::from_iter([path]))],
        )
    }
}
