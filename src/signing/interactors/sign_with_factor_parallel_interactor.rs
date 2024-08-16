use crate::prelude::*;

#[async_trait::async_trait]
pub trait UseFactorParallelInteractor {
    type Request;
    type Outcome;
    async fn use_factor_source(&self, request: Self::Request) -> Result<Self::Outcome>;
}

/// A interactor for a factor source kind which supports *Batch* usage of
/// multiple factor sources in parallel.
///
/// Most FactorSourceKinds does in fact NOT support parallel usage,
/// e.g. signing using multiple factors sources at once, but some do,
/// typically the DeviceFactorSource does, i.e. we can load multiple
/// mnemonics from secure storage in one go and sign with all of them
/// "in parallel".
///
/// This is a bit of a misnomer, as we don't actually use them in parallel,
/// but rather we iterate through all mnemonics and derive public keys/
/// or sign a payload with each of them in sequence
///
/// The user does not have the ability to SKIP a certain factor source,
/// instead either ALL factor sources are used to sign the transactions
/// or none.
///
/// Example of a Parallel Batch Signing Driver is that for DeviceFactorSource.
#[async_trait::async_trait]
pub trait SignWithFactorParallelInteractor {
    async fn sign(
        &self,
        request: ParallelBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>>;
}

impl<T: SignWithFactorParallelInteractor> UseFactorParallelInteractor for T {
    type Request = ParallelBatchSigningRequest;
    type Outcome = SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>;

    async fn use_factor_source(&self, request: Self::Request) -> Result<Self::Outcome> {
        self.sign(request).await
    }
}
