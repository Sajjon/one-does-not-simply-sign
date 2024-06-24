use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaybeSignedTransactions {
    /// Collection of transactions which might be signed or not.
    transactions: IndexMap<IntentHash, IndexSet<HDSignature>>,
}
impl MaybeSignedTransactions {
    pub fn new(transactions: IndexMap<IntentHash, IndexSet<HDSignature>>) -> Self {
        Self { transactions }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SignaturesOutcome {
    successful_transactions: MaybeSignedTransactions,
    failed_transactions: MaybeSignedTransactions,
}
impl SignaturesOutcome {
    pub fn new(
        successful_transactions: MaybeSignedTransactions,
        failed_transactions: MaybeSignedTransactions,
    ) -> Self {
        Self {
            successful_transactions,
            failed_transactions,
        }
    }
}
