use crate::prelude::*;

/// The response of a batch signing request, either a Parallel or Serial signing
/// request, matters not, because the goal is to have signed all transactions with
/// enough keys (derivation paths) needed for it to be valid when submitted to the
/// Radix network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BatchSigningResponse {
    /// The `IntentHash` should match the `intent_hash` of each HDSignature.
    signatures: HashMap<IntentHash, IndexSet<HDSignature>>,
}
