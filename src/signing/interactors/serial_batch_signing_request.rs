use crate::prelude::*;

/// A batch signing request used with a SignWithFactorSerialInteractor, containing
/// a collection of transactions to sign with multiple keys (derivation paths),
/// and a collection of transactions which would be invalid if the user skips
/// signing with this factor source.
#[derive(derive_more::Debug)]
#[debug("input: {:#?}", input)]
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
