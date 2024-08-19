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

fn derive_serially(
    request: SerialBatchKeyDerivationRequest,
) -> (FactorSourceID, IndexSet<FactorInstance>) {
    let factor_source_id = &request.factor_source_id;
    let instances = request
        .derivation_paths
        .into_iter()
        .map(|p| FactorInstance::mocked_with(p, factor_source_id))
        .collect::<IndexSet<_>>();

    (*factor_source_id, instances)
}

#[async_trait::async_trait]
impl DeriveKeyWithFactorParallelInteractor for TestDerivationParallelInteractor {
    async fn derive(
        &self,
        request: ParallelBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse> {
        let pairs = request
            .per_factor_source
            .into_iter()
            .map(|(_, v)| derive_serially(v))
            .collect::<IndexMap<_, _>>();
        Ok(BatchDerivationResponse::new(pairs))
    }
}

pub struct TestDerivationSerialInteractor;

#[async_trait::async_trait]
impl DeriveKeyWithFactorSerialInteractor for TestDerivationSerialInteractor {
    async fn derive(
        &self,
        request: SerialBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse> {
        let pair = derive_serially(request);
        Ok(BatchDerivationResponse::new(IndexMap::from_iter([pair])))
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
