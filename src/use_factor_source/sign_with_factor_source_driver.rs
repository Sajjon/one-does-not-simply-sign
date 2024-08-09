use crate::prelude::*;

pub type BatchSignTransactionsResponse = BatchUseFactorSourceResponse<IntentHash, HDSignature>;

pub type BatchSignTransactionsRequest = BatchUseFactorSourceRequest<IntentHash, HDPublicKey>;

#[async_trait::async_trait]
pub trait SignWithFactorSourceDriver:
    UseFactorSourceDriver<IntentHash, HDPublicKey, HDSignature>
{
    /// Produces many signatures for many entities from many factor sources for many transactions.
    async fn batch_sign_transactions(
        &self,
        request: BatchSignTransactionsRequest,
    ) -> Result<UseFactorsAction<BatchSignTransactionsResponse>>;
}

#[async_trait::async_trait]
impl<T: SignWithFactorSourceDriver + std::marker::Sync>
    UseFactorSourceDriver<IntentHash, HDPublicKey, HDSignature> for T
{
    async fn use_factors(
        &self,
        request: BatchUseFactorSourceRequest<IntentHash, HDPublicKey>,
    ) -> Result<UseFactorsAction<BatchUseFactorSourceResponse<IntentHash, HDSignature>>> {
        self.batch_sign_transactions(request).await
    }
}