use crate::prelude::*;

pub struct TestDerivationInteractors;

#[derive(Default, Clone, Debug)]
pub struct StatelessDummyIndices;

impl UsedDerivationIndices for StatelessDummyIndices {
    fn next_derivation_index_with_request(
        &self,
        request: CreateNextDerivationPathRequest,
    ) -> DerivationIndex {
        request.key_space.range().start
    }
}

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
    pub fn new_test_with_factor_sources(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        derivation_paths: impl IntoIterator<Item = (FactorSourceID, IndexSet<DerivationPath>)>,
    ) -> Self {
        Self::new(
            all_factor_sources_in_profile.into_iter().collect(),
            derivation_paths.into_iter().collect(),
            Arc::new(TestDerivationInteractors),
        )
    }

    pub fn new_test(
        derivation_paths: impl IntoIterator<Item = (FactorSourceID, IndexSet<DerivationPath>)>,
    ) -> Self {
        Self::new_test_with_factor_sources(FactorSource::all(), derivation_paths)
    }

    pub fn with(
        factor_source: &FactorSource,
        network_id: NetworkID,
        key_kind: KeyKind,
        entity_kind: EntityKind,
        key_space: KeySpace,
    ) -> Self {
        let indices = StatelessDummyIndices;
        let path = indices.next_derivation_path(
            factor_source.clone().id,
            network_id,
            key_kind,
            entity_kind,
            key_space,
        );
        Self::new_test_with_factor_sources(
            [factor_source.clone()],
            [(factor_source.id, IndexSet::from_iter([path]))],
        )
    }
}
