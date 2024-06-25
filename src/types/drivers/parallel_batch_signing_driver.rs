use crate::prelude::*;

/// A collection of factor sources to use to sign, transactions with multiple keys
/// (derivations paths).
pub struct ParallelBatchSigningRequest {
    /// Per factor source, a set of transactions to sign, with
    /// multiple derivations paths.
    pub per_factor_source: IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>,
    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source.
    pub invalid_transactions_if_skipped: IndexSet<InvalidTransactionIfSkipped>,
}
impl ParallelBatchSigningRequest {
    pub fn new(
        per_factor_source: IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>,
        invalid_transactions_if_skipped: IndexSet<InvalidTransactionIfSkipped>,
    ) -> Self {
        Self {
            per_factor_source,
            invalid_transactions_if_skipped,
        }
    }
}

/// A driver for a factor source kind which supports performing:
/// *Batch* signing *in parallel*.
///
/// Most FactorSourceKinds does in fact NOT support parallel signing,
/// i.e. signing using multiple factors sources at once, but some do,
/// typically the DeviceFactorSource does, i.e. we can load multiple
/// mnemonics from secure storage in one go and sign with all of them
/// "in parallel".
///
/// This is a bit of a misnomer, as we don't actually sign in parallel,
/// but rather we iterate through all mnemonics and sign the payload
/// with each of them in sequence
///
/// The user does not have the ability to SKIP a certain factor source,
/// instead either ALL factor sources are used to sign the transactions
/// or none.
///
/// Example of a Parallel Batch Signing Driver is that for DeviceFactorSource.
#[async_trait::async_trait]
pub trait ParallelBatchSigningDriver {
    async fn sign(
        &self,
        request: ParallelBatchSigningRequest,
    ) -> SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>;
}

#[cfg(test)]
pub struct TestParallelBatchSigningDriver {
    pub simulated_user: SimulatedUser,
}
#[cfg(test)]
impl TestParallelBatchSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}
#[cfg(test)]
#[async_trait]
impl ParallelBatchSigningDriver for TestParallelBatchSigningDriver {
    async fn sign(
        &self,
        request: ParallelBatchSigningRequest,
    ) -> SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse> {
        match self
            .simulated_user
            .sign_or_skip(request.invalid_transactions_if_skipped)
        {
            SigningUserInput::Sign => {
                let signatures = request
                    .per_factor_source
                    .iter()
                    .map(|(k, v)| {
                        let value = v
                            .per_transaction
                            .iter()
                            .flat_map(|x| {
                                x.owned_factor_instances
                                    .iter()
                                    .map(|y| {
                                        HDSignature::new(
                                            x.intent_hash.clone(),
                                            Signature,
                                            y.clone(),
                                        )
                                    })
                                    .collect_vec()
                            })
                            .collect::<IndexSet<HDSignature>>();
                        (k.clone(), value)
                    })
                    .collect::<IndexMap<FactorSourceID, IndexSet<HDSignature>>>();

                SignWithFactorSourceOrSourcesOutcome::Signed(BatchSigningResponse::new(signatures))
            }
            SigningUserInput::Skip => SignWithFactorSourceOrSourcesOutcome::Skipped(
                request
                    .per_factor_source
                    .keys()
                    .into_iter()
                    .map(|x| x.clone())
                    .collect_vec(),
            ),
        }
    }
}

pub struct ParallelBatchSigningClient {
    driver: Arc<dyn ParallelBatchSigningDriver>,
}

impl ParallelBatchSigningClient {
    pub fn new(driver: Arc<dyn ParallelBatchSigningDriver>) -> Self {
        Self { driver }
    }
    pub async fn sign(
        &self,
        request: ParallelBatchSigningRequest,
    ) -> SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse> {
        self.driver.sign(request).await
    }
}
