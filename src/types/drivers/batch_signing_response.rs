use crate::prelude::*;

/// The response of a batch signing request, either a Parallel or Serial signing
/// request, matters not, because the goal is to have signed all transactions with
/// enough keys (derivation paths) needed for it to be valid when submitted to the
/// Radix network.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BatchSigningResponse {
    pub signatures: IndexMap<FactorSourceID, IndexSet<HDSignature>>,
}
impl BatchSigningResponse {
    pub fn new(signatures: IndexMap<FactorSourceID, IndexSet<HDSignature>>) -> Self {
        Self { signatures }
    }
}
