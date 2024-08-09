use crate::prelude::*;

pub trait BaseUseFactorSourceDriver {
    fn supports(&self, factor_source_kind: FactorSourceKind) -> bool;
}

#[async_trait::async_trait]
pub trait UseFactorSourceDriver<
    ID,
    Path,
    Product,
    Request = BatchUseFactorSourceRequest<ID, Path>,
    Response = BatchUseFactorSourceResponse<ID, Product>,
>: BaseUseFactorSourceDriver where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    async fn use_factor(&self, request: Request) -> Result<Response>;
}
