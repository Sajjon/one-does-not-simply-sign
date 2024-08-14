use crate::prelude::*;

pub type BatchDerivePublicKeysResponse = BatchUseFactorSourceResponse<DeriveKeyID, HDPublicKey>;

pub type BatchDerivePublicKeysRequest = BatchUseFactorSourceRequest<DeriveKeyID, DerivationPath>;

#[async_trait::async_trait]
pub trait DeriveKeysWithFactorSourceDriver:
    UseFactorSourceDriver<(), DerivationPath, HDPublicKey>
{
    /// Derives many keys from many factor sources for many entities.
    async fn batch_derive_public_keys(
        &self,
        request: BatchDerivePublicKeysRequest,
    ) -> Result<BatchDerivePublicKeysResponse>;
}

#[async_trait::async_trait]
impl<T: DeriveKeysWithFactorSourceDriver + std::marker::Sync>
    UseFactorSourceDriver<DeriveKeyID, DerivationPath, HDPublicKey> for T
{
    async fn use_factors(
        &self,
        request: BatchDerivePublicKeysRequest,
    ) -> Result<UseFactorsAction<BatchDerivePublicKeysRequest, BatchDerivePublicKeysResponse>> {
        let response = self.batch_derive_public_keys(request).await?;

        Ok(UseFactorsAction::Used(response))
    }
}
