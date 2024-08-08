use crate::prelude::*;

/// === FIA ===

pub struct FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    phantom_id: PhantomData<ID>,
    phantom_path: PhantomData<Path>,
    phantom_product: PhantomData<Product>,
}

/// ===== Public =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    pub fn new(
        request: BatchUseFactorSourceRequest<ID, Path>,
        factor_sources: Vec<FactorSource>,
    ) -> Result<Self> {
        todo!()
    }

    pub async fn accumulate(&self) -> Result<BatchUseFactorSourceResponse<ID, Product>> {
        todo!()
    }
}
