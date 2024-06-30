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
    intent_hash: IntentHash,
    owned_factor_instance: OwnedFactorInstance,
}
impl SerialSingleSigningRequestPartial {
    pub fn signature_input(&self) -> HDSignatureInput {
        HDSignatureInput::new(self.intent_hash.clone(), self.owned_factor_instance.clone())
    }
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
pub trait SerialSingleUseFactorSourceDriver {
    async fn sign(
        &self,
        request: SerialSingleSigningRequestFull,
    ) -> SignWithFactorSourceOrSourcesOutcome<HDSignature>;
}

pub struct SerialSingleUseFactorSourceClient {
    driver: Arc<dyn SerialSingleUseFactorSourceDriver>,
}
impl SerialSingleUseFactorSourceClient {
    pub fn new(driver: Arc<dyn SerialSingleUseFactorSourceDriver>) -> Self {
        Self { driver }
    }
    pub async fn sign(
        &self,
        request: SerialSingleSigningRequestFull,
    ) -> SignWithFactorSourceOrSourcesOutcome<HDSignature> {
        self.driver.sign(request).await
    }
}
