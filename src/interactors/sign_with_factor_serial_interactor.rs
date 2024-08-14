use crate::prelude::*;

/// A interactor for a factor source kind which support performing
/// *Batch* signing *serially*.
///
/// Meaning we initiate and prompt user for signing with one factor source
/// at a time, where each signing operation is support batch signing, that is
/// signing multiple transactions each with multiple keys (derivations paths).
///
/// The user might chose to SKIP the current factor source, and move on to the
/// next one.
///
/// Example of a Serial Batch Signing Driver is SecurityQuestionsFactorSource,
/// where it does not make any sense to let user in parallel answer multiple
/// questions from different security questions factor sources (in fact we
/// might not even even allow multiple SecurityQuestionsFactorSources to be used).
#[async_trait]
pub trait SignWithFactorSerialInteractor {
    async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>>;
}
