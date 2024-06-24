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
    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.transactions
            .values()
            .flat_map(|v| v.iter())
            .cloned()
            .collect()
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
    /// All signatures from both successful transactions and failed transactions.
    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.successful_transactions
            .all_signatures()
            .union(&self.failed_transactions.all_signatures())
            .cloned()
            .collect()
    }
}
