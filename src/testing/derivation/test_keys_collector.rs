use crate::prelude::*;

pub struct TestDerivationInteractors {
    pub parallel: Arc<dyn DeriveKeyWithFactorParallelInteractor + Send + Sync>,
    pub serial: Arc<dyn DeriveKeyWithFactorSerialInteractor + Send + Sync>,
}
impl TestDerivationInteractors {
    pub fn new(
        parallel: impl DeriveKeyWithFactorParallelInteractor + Send + Sync + 'static,
        serial: impl DeriveKeyWithFactorSerialInteractor + Send + Sync + 'static,
    ) -> Self {
        Self {
            parallel: Arc::new(parallel),
            serial: Arc::new(serial),
        }
    }
}

impl TestDerivationInteractors {
    pub fn fail() -> Self {
        Self::new(
            TestDerivationParallelInteractor::fail(),
            TestDerivationSerialInteractor::fail(),
        )
    }
}
impl Default for TestDerivationInteractors {
    fn default() -> Self {
        Self::new(
            TestDerivationParallelInteractor::default(),
            TestDerivationSerialInteractor::default(),
        )
    }
}

impl KeysCollectingInteractors for TestDerivationInteractors {
    fn interactor_for(&self, kind: FactorSourceKind) -> KeyDerivationInteractor {
        match kind {
            FactorSourceKind::Device => KeyDerivationInteractor::parallel(self.parallel.clone()),
            _ => KeyDerivationInteractor::serial(self.serial.clone()),
        }
    }
}

pub struct TestDerivationParallelInteractor {
    handle: fn(SerialBatchKeyDerivationRequest) -> Result<IndexSet<FactorInstance>>,
}
impl TestDerivationParallelInteractor {
    pub fn new(
        handle: fn(SerialBatchKeyDerivationRequest) -> Result<IndexSet<FactorInstance>>,
    ) -> Self {
        Self { handle }
    }
    pub fn fail() -> Self {
        Self::new(|_| Err(CommonError::Failure))
    }
    fn derive(&self, request: SerialBatchKeyDerivationRequest) -> Result<IndexSet<FactorInstance>> {
        (self.handle)(request)
    }
}
impl Default for TestDerivationParallelInteractor {
    fn default() -> Self {
        Self::new(do_derive_serially)
    }
}

fn do_derive_serially(
    request: SerialBatchKeyDerivationRequest,
) -> Result<IndexSet<FactorInstance>> {
    let factor_source_id = &request.factor_source_id;
    let instances = request
        .derivation_paths
        .into_iter()
        .map(|p| FactorInstance::mocked_with(p, factor_source_id))
        .collect::<IndexSet<_>>();

    Ok(instances)
}

#[async_trait::async_trait]
impl DeriveKeyWithFactorParallelInteractor for TestDerivationParallelInteractor {
    async fn derive(
        &self,
        request: ParallelBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse> {
        let pairs_result: Result<IndexMap<FactorSourceID, IndexSet<FactorInstance>>> = request
            .per_factor_source
            .into_iter()
            .map(|(k, r)| {
                let instances = self.derive(r);
                instances.map(|i| (k, i))
            })
            .collect();
        let pairs = pairs_result?;
        Ok(BatchDerivationResponse::new(pairs))
    }
}

pub struct TestDerivationSerialInteractor {
    handle: fn(SerialBatchKeyDerivationRequest) -> Result<IndexSet<FactorInstance>>,
}
impl TestDerivationSerialInteractor {
    pub fn new(
        handle: fn(SerialBatchKeyDerivationRequest) -> Result<IndexSet<FactorInstance>>,
    ) -> Self {
        Self { handle }
    }
    pub fn fail() -> Self {
        Self::new(|_| Err(CommonError::Failure))
    }
    fn derive(&self, request: SerialBatchKeyDerivationRequest) -> Result<IndexSet<FactorInstance>> {
        (self.handle)(request)
    }
}
impl Default for TestDerivationSerialInteractor {
    fn default() -> Self {
        Self::new(do_derive_serially)
    }
}

#[async_trait::async_trait]
impl DeriveKeyWithFactorSerialInteractor for TestDerivationSerialInteractor {
    async fn derive(
        &self,
        request: SerialBatchKeyDerivationRequest,
    ) -> Result<BatchDerivationResponse> {
        let instances = self.derive(request.clone())?;
        Ok(BatchDerivationResponse::new(IndexMap::from_iter([(
            request.factor_source_id,
            instances,
        )])))
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
            Arc::new(TestDerivationInteractors::default()),
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
        key_kind: CAP26KeyKind,
        entity_kind: CAP26EntityKind,
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
