use crate::prelude::*;

/// The outcome of a SignaturesCollector, containing a collection for transactions
/// which would be successful if submitted to the network (from a signatures point of view)
/// and a collection of transactions which would fail if submitted to the network,
/// since not enough signatures have been gathered. And a collection of factor sources
/// which were skipped.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SignaturesOutcome {
    /// A potentially empty collection of transactions which which would be
    /// successful if submitted to the network (from a signatures point of view).
    ///
    /// Potentially empty
    successful_transactions: MaybeSignedTransactions,

    /// A collection of transactions which would fail if submitted to the network,
    /// since not enough signatures have been gathered.
    ///
    /// Potentially empty
    failed_transactions: MaybeSignedTransactions,

    /// List of ids of all factor sources which failed.
    skipped_factor_sources: IndexSet<FactorSourceID>,
}

impl SignaturesOutcome {
    /// # Panics
    /// Panics if the `successful_transactions` or `failed_transactions` shared
    /// either any transaction intent hash, or any signature.
    pub fn new(
        successful_transactions: MaybeSignedTransactions,
        failed_transactions: MaybeSignedTransactions,
        skipped_factor_sources: impl IntoIterator<Item = FactorSourceID>,
    ) -> Self {
        let skipped_factor_sources = skipped_factor_sources.into_iter().collect::<IndexSet<_>>();
        let successful_hashes: IndexSet<IntentHash> = successful_transactions
            .transactions
            .keys()
            .cloned()
            .collect();
        let failure_hashes: IndexSet<IntentHash> =
            failed_transactions.transactions.keys().cloned().collect();

        let valid = successful_hashes
            .intersection(&failure_hashes)
            .collect_vec()
            .is_empty();

        assert!(
            valid,
            "Discrepancy, found intent hash in both successful and failed transactions, this is a programmer error."
        );

        Self {
            successful_transactions,
            failed_transactions,
            skipped_factor_sources,
        }
    }

    pub fn successful(&self) -> bool {
        self.failed_transactions.is_empty()
    }

    pub fn signatures_of_successful_transactions(&self) -> IndexSet<HDSignature> {
        self.successful_transactions.all_signatures()
    }

    pub fn successful_transactions(&self) -> Vec<SignedTransaction> {
        self.successful_transactions.clone().transactions()
    }

    pub fn skipped_factor_sources(&self) -> IndexSet<FactorSourceID> {
        self.skipped_factor_sources.clone()
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

#[cfg(test)]
mod tests {

    use super::*;
    type Sut = SignaturesOutcome;

    #[test]
    #[should_panic(
        expected = "Discrepancy, found intent hash in both successful and failed transactions, this is a programmer error."
    )]
    fn new_panics_if_intent_hash_is_in_both_failed_and_success_collection() {
        Sut::new(
            MaybeSignedTransactions::sample(),
            MaybeSignedTransactions::sample(),
            [],
        );
    }
}
