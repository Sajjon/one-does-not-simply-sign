use crate::prelude::*;

pub trait BaseUseFactorSourceDriver {
    fn supports(&self, factor_source_kind: FactorSourceKind) -> bool;
}

#[derive(Debug, Clone)]
pub enum UseFactorsAction<Response> {
    Skipped,
    Used(Response),
}

impl<T> UseFactorsAction<T> {
    pub fn skipped(&self) -> bool {
        match self {
            Self::Skipped => true,
            Self::Used(_) => false,
        }
    }
}

/// Implementors SHOULD handle failures and retry on their end. To be clear,
/// FIA (or any of its subparts) does NOT handle failures and ask for retries.
/// If `use_factors` returns `Err`, it means that implementor already have
/// informed user of failure and asked for retry, or that the failure is
/// unrecoverable / critical.
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
    /// Retries is NOT handled by FIA, so implementors SHOULD prompt user to
    /// retry if e.g. signing with Ledger failed, and only if user does not wanna
    /// retry or if host cannot retry, should implementor return `Err`.
    async fn use_factors(&self, request: Request) -> Result<UseFactorsAction<Response>>;
}
