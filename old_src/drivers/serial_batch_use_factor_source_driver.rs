use crate::prelude::*;

/// A batch signing request used with a SerialBatchUseFactorSourceDriver, containing
/// a collection of transactions to sign with multiple keys (derivation paths),
/// and a collection of transactions which would be invalid if the user skips
/// signing with this factor source.
pub struct SerialBatchSigningRequest {
    pub input: BatchTXBatchKeySigningRequest,
    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source.
    pub invalid_transactions_if_skipped: Vec<InvalidTransactionIfSkipped>,
}
impl SerialBatchSigningRequest {
    pub fn new(
        input: BatchTXBatchKeySigningRequest,
        invalid_transactions_if_skipped: Vec<InvalidTransactionIfSkipped>,
    ) -> Self {
        Self {
            input,
            invalid_transactions_if_skipped,
        }
    }
}

/// A driver for a factor source kind which support performing
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
pub trait SerialBatchUseFactorSourceDriver: IsUseFactorSourcesDriver {
    async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>>;
}
