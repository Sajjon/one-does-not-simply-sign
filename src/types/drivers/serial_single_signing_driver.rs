use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SerialSingleSigningRequestFull {
    pub input: SerialSingleSigningRequestPartial,
    pub invalid_transactions_if_skipped: Vec<InvalidTransactionIfSkipped>,
}
impl SerialSingleSigningRequestFull {
    pub fn new(
        input: SerialSingleSigningRequestPartial,
        invalid_transactions_if_skipped: IndexSet<InvalidTransactionIfSkipped>,
    ) -> Self {
        Self {
            input,
            invalid_transactions_if_skipped: invalid_transactions_if_skipped
                .into_iter()
                .collect_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SerialSingleSigningRequestPartial {
    pub factor_source_id: FactorSourceID,
    pub intent_hash: IntentHash,
    pub owned_factor_instance: OwnedFactorInstance,
}
impl SerialSingleSigningRequestPartial {
    pub fn new(
        factor_source_id: FactorSourceID,
        intent_hash: IntentHash,
        owned_factor_instance: OwnedFactorInstance,
    ) -> Self {
        Self {
            factor_source_id,
            intent_hash,
            owned_factor_instance,
        }
    }
}

/// A driver for factor source kinds which cannot sign multiple transactions
/// nor sign a single transaction with multiple keys (derivation paths).
///
/// Example of a Serial Single Signing Driver *might* be `Arculus` - we
/// do not yet know.
#[async_trait]
pub trait SerialSingleSigningDriver {
    async fn sign(
        &self,
        request: SerialSingleSigningRequestFull,
    ) -> SignWithFactorSourceOrSourcesOutcome<HDSignature>;
}

pub struct SerialSingleSigningClient {
    driver: Arc<dyn SerialSingleSigningDriver>,
}
impl SerialSingleSigningClient {
    pub fn new(driver: Arc<dyn SerialSingleSigningDriver>) -> Self {
        Self { driver }
    }
    pub async fn sign(
        &self,
        request: SerialSingleSigningRequestFull,
    ) -> SignWithFactorSourceOrSourcesOutcome<HDSignature> {
        self.driver.sign(request).await
    }
}

#[cfg(test)]
pub struct TestSerialSingleSigningDriver {
    pub simulated_user: SimulatedUser,
}
#[cfg(test)]
impl TestSerialSingleSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[cfg(test)]
#[async_trait]
impl SerialSingleSigningDriver for TestSerialSingleSigningDriver {
    async fn sign(
        &self,
        request: SerialSingleSigningRequestFull,
    ) -> SignWithFactorSourceOrSourcesOutcome<HDSignature> {
        match self
            .simulated_user
            .sign_or_skip(request.invalid_transactions_if_skipped)
        {
            SigningUserInput::Sign => {
                SignWithFactorSourceOrSourcesOutcome::signed(HDSignature::new(
                    request.input.intent_hash,
                    Signature,
                    request.input.owned_factor_instance,
                ))
            }
            SigningUserInput::Skip => SignWithFactorSourceOrSourcesOutcome::skipped_factor_source(
                request.input.factor_source_id,
            ),
        }
    }
}
