use crate::prelude::*;

/// A batch signing request used with a SerialBatchSigningDriver, containing
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
pub trait SerialBatchSigningDriver {
    async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>;
}

pub struct SerialBatchSigningClient {
    driver: Arc<dyn SerialBatchSigningDriver>,
}
impl SerialBatchSigningClient {
    pub fn new(driver: Arc<dyn SerialBatchSigningDriver>) -> Self {
        Self { driver }
    }
    pub async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse> {
        self.driver.sign(request).await
    }
}

#[cfg(test)]
pub struct TestSerialBatchSigningDriver {
    pub simulated_user: SimulatedUser,
}
#[cfg(test)]
impl TestSerialBatchSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[cfg(test)]
#[async_trait]
impl SerialBatchSigningDriver for TestSerialBatchSigningDriver {
    async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse> {
        let invalid_transactions_if_skipped = request.invalid_transactions_if_skipped;
        match self
            .simulated_user
            .sign_or_skip(invalid_transactions_if_skipped)
        {
            SigningUserInput::Sign => {
                let signatures = request
                    .input
                    .per_transaction
                    .into_iter()
                    .map(|r| {
                        let key = r.factor_source_id;
                        let value = r
                            .owned_factor_instances
                            .into_iter()
                            .map(|f| HDSignature::new(r.intent_hash.clone(), Signature, f.clone()))
                            .collect::<IndexSet<_>>();
                        (key, value)
                    })
                    .collect::<IndexMap<FactorSourceID, IndexSet<HDSignature>>>();
                let response = BatchSigningResponse::new(signatures);
                SignWithFactorSourceOrSourcesOutcome::Signed(response)
            }
            SigningUserInput::Skip => {
                SignWithFactorSourceOrSourcesOutcome::Skipped(vec![request.input.factor_source_id])
            }
        }
    }
}
