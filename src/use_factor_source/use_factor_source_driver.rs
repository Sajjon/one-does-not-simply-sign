use crate::prelude::*;

#[async_trait::async_trait]
pub trait UseFactorSourceDriver<ID, Path, Product>
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
