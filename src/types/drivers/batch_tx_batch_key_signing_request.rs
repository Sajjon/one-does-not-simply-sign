use crate::prelude::*;

/// A batch of keys (derivation paths) all being factor instances of a FactorSource
/// with id `factor_source_id` to sign a single transaction with, which hash
/// is `intent_hash`.
#[derive(Derivative)]
#[derivative(PartialEq, Eq, Clone, Debug, Hash)]
pub struct BatchKeySigningRequest {
    /// Hash to sign
    pub intent_hash: IntentHash,

    /// ID of factor to use to sign
    pub factor_source_id: FactorSourceID,

    /// The derivation paths to use to derive the private keys to sign with. The
    /// `factor_source_id` of each item must match `factor_source_id`.
    #[derivative(Hash = "ignore")]
    pub owned_factor_instances: IndexSet<OwnedFactorInstance>,
}
impl BatchKeySigningRequest {
    pub fn new(
        intent_hash: IntentHash,
        factor_source_id: FactorSourceID,
        owned_factor_instances: IndexSet<OwnedFactorInstance>,
    ) -> Self {
        assert!(owned_factor_instances
            .iter()
            .all(|f| f.factor_instance.factor_source_id == factor_source_id));
        Self {
            intent_hash,
            factor_source_id,
            owned_factor_instances,
        }
    }
}

/// A batch of transactions each batching over multiple keys (derivation paths)
/// to sign each transaction with.
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct BatchTXBatchKeySigningRequest {
    /// The ID of the factor source used to sign each per_transaction
    pub factor_source_id: FactorSourceID,

    // The `factor_source_id` of each item must match `self.factor_source_id`.
    pub per_transaction: Vec<BatchKeySigningRequest>,
}

impl BatchTXBatchKeySigningRequest {
    pub fn new(
        factor_source_id: FactorSourceID,
        per_transaction: IndexSet<BatchKeySigningRequest>,
    ) -> Self {
        assert!(per_transaction
            .iter()
            .all(|f| f.factor_source_id == factor_source_id));
        Self {
            factor_source_id,
            per_transaction: per_transaction.into_iter().collect(),
        }
    }
}
