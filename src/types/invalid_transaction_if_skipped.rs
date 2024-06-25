use crate::prelude::*;

/// A list of entities which would fail in a transaction if we would
/// skip signing with a certain factor source
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct InvalidTransactionIfSkipped {
    /// The intent hash of the transaction which would be invalid if we skipped
    /// signing with a certain factor source
    pub intent_hash: IntentHash,

    /// The entities in the transaction which would fail auth.
    pub entities_which_would_fail_auth: Vec<AccountAddressOrIdentityAddress>,
}

impl InvalidTransactionIfSkipped {
    /// Constructs a new `InvalidTransactionIfSkipped`.
    pub fn new(
        intent_hash: IntentHash,
        entities_which_would_fail_auth: Vec<AccountAddressOrIdentityAddress>,
    ) -> Self {
        Self {
            intent_hash,
            entities_which_would_fail_auth,
        }
    }
}
