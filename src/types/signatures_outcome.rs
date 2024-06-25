use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaybeSignedTransactions {
    /// Collection of transactions which might be signed or not.
    transactions: IndexMap<IntentHash, IndexSet<HDSignature>>,
}
impl MaybeSignedTransactions {
    pub fn empty() -> Self {
        Self {
            transactions: IndexMap::new(),
        }
    }
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub fn add_signatures(&mut self, txid: IntentHash, signatures: IndexSet<HDSignature>) {
        if let Some(ref mut sigs) = self.transactions.get_mut(&txid) {
            sigs.extend(signatures);
        } else {
            self.transactions.insert(txid, signatures);
        }
    }
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

    pub fn successful(&self) -> bool {
        self.failed_transactions.is_empty()
    }

    pub fn signatures_of_successful_transactions(&self) -> IndexSet<HDSignature> {
        self.successful_transactions.all_signatures()
    }

    pub fn signatures_of_failed_transactions(&self) -> IndexSet<HDSignature> {
        self.failed_transactions.all_signatures()
    }

    /// All signatures from both successful transactions and failed transactions.
    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.signatures_of_successful_transactions()
            .union(&self.signatures_of_failed_transactions())
            .cloned()
            .collect()
    }
}
