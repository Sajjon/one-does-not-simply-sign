use super::*;
use crate::prelude::*;

pub(super) struct FiaState<ID, Path, Product>
where
    ID: Hash + Send + Sync,
    Path: HasDerivationPath + Send + Sync,
    Product: HasHDPublicKey + Send + Sync,
{
    phantom_id: PhantomData<ID>,
    phantom_path: PhantomData<Path>,
    phantom_product: PhantomData<Product>,
    /// If this FIA is used to derive public keys we cannot skip any
    /// factor source, we need to derive ALL keys, so this variable will
    /// be `false`. However, for transaction signing, this variable will
    /// be `true`, since we **might** be able to skip *some* factor sources
    /// and have valid signed transactions with e.g. just override factors.
    pub(super) supports_skipping_of_factor_sources: bool,

    /// Factor Sources Left to use. When this is Vec empty, we are done.
    pub(super) factor_sources: Vec<FactorSource>,
}

impl<ID, Path, Product> FiaState<ID, Path, Product>
where
    ID: Hash + Send + Sync + 'static,
    Path: HasDerivationPath + Send + Sync,
    Product: HasHDPublicKey + Send + Sync,
{
    fn _invalid_if_skipped(&self, factor_sources: &[FactorSource]) -> Vec<ID> {
        assert!(self.supports_skipping_of_factor_sources);
        todo!()
    }

    fn _request_for(&self, factor_sources: &[FactorSource]) -> Self::DriverRequest {
        assert_eq!(
            factor_sources
                .iter()
                .map(|f| f.kind)
                .collect::<HashSet<FactorSourceKind>>()
                .len(),
            1
        );
        let supports_skipping = self.supports_skipping_of_factor_sources;
        let inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>> = HashMap::new();
        if supports_skipping {
            Self::DriverRequest::new_skippable(self._invalid_if_skipped(factor_sources), inputs)
        } else {
            Self::DriverRequest::new_unskippable(inputs)
        }
    }
}

impl<ID, Path, Product> FiaState<ID, Path, Product>
where
    ID: Hash + Send + Sync + 'static,
    Path: HasDerivationPath + Send + Sync,
    Product: HasHDPublicKey + Send + Sync,
{
    pub(super) fn new(
        supports_skipping_of_factor_sources: bool,
        factor_sources: Vec<FactorSource>,
    ) -> Self {
        Self {
            supports_skipping_of_factor_sources,
            phantom_id: PhantomData,
            phantom_path: PhantomData,
            phantom_product: PhantomData,
            factor_sources,
        }
    }

    pub(super) type DriverRequest = BatchUseFactorSourceRequest<ID, Path>;
    pub(super) type DriverResponse = BatchUseFactorSourceResponse<ID, Product>;

    pub(super) fn next_factor_sources(&self) -> Option<FactorSourcesOfKind> {
        todo!()
    }

    pub(super) fn parallel_request_for(
        &self,
        factor_sources_of_kind: &FactorSourcesOfKind,
    ) -> Self::DriverRequest {
        assert!(factor_sources_of_kind.kind.supports_parallelism());
        self._request_for(&factor_sources_of_kind.factor_sources)
    }

    pub(super) fn serial_request_for(&self, factor_source: &FactorSource) -> Self::DriverRequest {
        assert!(!factor_source.kind.supports_parallelism());
        self._request_for(&[factor_source.clone()])
    }

    pub(super) fn handle_outcome(
        &mut self,
        response: UseFactorsAction<Self::DriverResponse>,
    ) -> Result<()> {
        todo!()
    }

    pub(super) fn accumulated_output(&self) -> Result<FiaOutput<ID, Product>> {
        todo!()
    }
}
