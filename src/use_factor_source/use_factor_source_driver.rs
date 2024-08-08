use crate::prelude::*;

pub trait BaseUseFactorSourceDriver {
    fn can_be_used_for(&self, factor_source: &FactorSource) -> bool;
}

#[async_trait::async_trait]
pub trait UseFactorSourceDriver<ID, Path, Product>: BaseUseFactorSourceDriver
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    async fn use_factor(
        &self,
        request: BatchUseFactorSourceRequest<ID, Path>,
    ) -> Result<BatchUseFactorSourceResponse<ID, Product>>;
}
