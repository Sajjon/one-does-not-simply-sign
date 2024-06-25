use crate::prelude::*;

impl HDSignature {
    pub fn new(
        intent_hash: IntentHash,
        signature: Signature,
        owned_factor_instance: OwnedFactorInstance,
    ) -> Self {
        Self {
            intent_hash,
            signature,
            owned_factor_instance,
        }
    }
}

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
pub enum SignWithFactorSourceOrSourcesOutcome<T> {
    #[debug("Signed: {:?}", _0)]
    Signed(T),
    #[debug("Skipped")]
    Skipped(Vec<FactorSourceID>),
}
