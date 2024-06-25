use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct InvalidTransactionIfSkipped {
    pub intent_hash: IntentHash,
    pub entities_which_would_fail_auth: Vec<AccountAddressOrIdentityAddress>,
}
impl InvalidTransactionIfSkipped {
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
