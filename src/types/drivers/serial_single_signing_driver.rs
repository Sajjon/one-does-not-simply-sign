use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SerialSingleSigningRequest {
    /// The ID of the factor source used to sign each per_transaction
    factor_source_id: FactorSourceID,

    intent_hash: IntentHash,

    owned_factor_instance: OwnedHDFactorInstance,
}

/// A driver for factor source kinds which cannot sign multiple transactions
/// nor sign a single transaction with multiple keys (derivation paths).
///
/// Example of a Serial Single Signing Driver *might* be `Arculus` - we
/// do not yet know.
#[async_trait]
pub trait SerialSingleSigningDriver {
    async fn sign(&self, request: SerialSingleSigningRequest) -> HDSignature;
}
