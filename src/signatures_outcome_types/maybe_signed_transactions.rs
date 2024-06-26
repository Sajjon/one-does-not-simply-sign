use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaybeSignedTransactions {
    /// Collection of transactions which might be signed or not.
    pub(super) transactions: IndexMap<IntentHash, IndexSet<HDSignature>>,
}

impl HasSampleValues for MaybeSignedTransactions {
    fn sample() -> Self {
        let tx_a = IntentHash::sample();
        let tx_a_input_x = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AccountAddressOrIdentityAddress::sample(),
                FactorInstance::new(0, FactorSourceID::sample()),
            ),
        );
        let tx_a_input_y = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AccountAddressOrIdentityAddress::sample(),
                FactorInstance::new(1, FactorSourceID::sample_other()),
            ),
        );
        let tx_a_sig_x = HDSignature::produced_signing_with_input(tx_a_input_x);
        let tx_a_sig_y = HDSignature::produced_signing_with_input(tx_a_input_y);

        let tx_b = IntentHash::sample_other();
        let tx_b_input_x = HDSignatureInput::new(
            tx_b.clone(),
            OwnedFactorInstance::new(
                AccountAddressOrIdentityAddress::sample(),
                FactorInstance::new(2, FactorSourceID::sample_third()),
            ),
        );
        let tx_b_input_y = HDSignatureInput::new(
            tx_b.clone(),
            OwnedFactorInstance::new(
                AccountAddressOrIdentityAddress::sample(),
                FactorInstance::new(3, FactorSourceID::sample_fourth()),
            ),
        );

        let tx_b_sig_x = HDSignature::produced_signing_with_input(tx_b_input_x);
        let tx_b_sig_y = HDSignature::produced_signing_with_input(tx_b_input_y);

        Self::new(
            [
                (tx_a, IndexSet::from_iter([tx_a_sig_x, tx_a_sig_y])),
                (tx_b, IndexSet::from_iter([tx_b_sig_x, tx_b_sig_y])),
            ]
            .into_iter()
            .collect::<IndexMap<IntentHash, IndexSet<HDSignature>>>(),
        )
    }

    fn sample_other() -> Self {
        let tx_a = IntentHash::sample_third();
        let tx_a_input_x = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AccountAddressOrIdentityAddress::sample(),
                FactorInstance::new(10, FactorSourceID::sample()),
            ),
        );
        let tx_a_input_y = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AccountAddressOrIdentityAddress::sample(),
                FactorInstance::new(11, FactorSourceID::sample_other()),
            ),
        );
        let tx_a_input_z = HDSignatureInput::new(
            tx_a.clone(),
            OwnedFactorInstance::new(
                AccountAddressOrIdentityAddress::sample(),
                FactorInstance::new(12, FactorSourceID::sample_third()),
            ),
        );
        let tx_a_sig_x = HDSignature::produced_signing_with_input(tx_a_input_x);
        let tx_a_sig_y = HDSignature::produced_signing_with_input(tx_a_input_y);
        let tx_a_sig_z = HDSignature::produced_signing_with_input(tx_a_input_z);

        Self::new(
            [(
                tx_a,
                IndexSet::from_iter([tx_a_sig_x, tx_a_sig_y, tx_a_sig_z]),
            )]
            .into_iter()
            .collect::<IndexMap<IntentHash, IndexSet<HDSignature>>>(),
        )
    }
}

impl MaybeSignedTransactions {
    fn new(transactions: IndexMap<IntentHash, IndexSet<HDSignature>>) -> Self {
        Self { transactions }
    }

    /// Constructs a new empty `MaybeSignedTransactions` which can be used
    /// as a "builder".
    pub fn empty() -> Self {
        Self::new(IndexMap::new())
    }

    /// Returns whether or not this `MaybeSignedTransactions` contains
    /// any transactions.
    pub fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    /// Validates that all values, all signatures, have the same `intent_hash`
    /// as its key.
    ///
    /// Also validates that the input of every signature is unique - to identify
    /// if the same signer has been used twice, would be a programmer error.
    ///
    /// # Panics
    /// Panics if any signature has a different `intent_hash` than its key.
    fn validate(&self) {
        for (intent_hash, signatures) in self.transactions.iter() {
            assert!(
                signatures.iter().all(|s| s.intent_hash() == intent_hash),
                "Discrepancy between intent hash and signature intent hash."
            );
        }
        let all_signatures = self.all_signatures();
        let all_signatures_count = all_signatures.len();
        let inputs = self
            .all_signatures()
            .iter()
            .map(|s| s.input.clone())
            .collect::<IndexSet<_>>();
        assert_eq!(
            all_signatures_count,
            inputs.len(),
            "Discrepancy, the same signer has been used twice."
        );
    }

    /// Inserts a set of signatures for transaction with `intent_hash`, if
    /// the transaction was already present, the signatures are added to the
    /// existing set, if the transaction was not already present a new set is
    /// created.
    ///
    /// # Panics
    /// Panics if any signature has a different `intent_hash` than its key.
    ///
    /// Panics if any signatures in `signature` is not new, that is, already present
    /// in `transactions`.
    pub fn add_signatures(&mut self, intent_hash: IntentHash, signatures: IndexSet<HDSignature>) {
        if let Some(ref mut sigs) = self.transactions.get_mut(&intent_hash) {
            let old_count = sigs.len();
            let delta_count = signatures.len();
            sigs.extend(signatures);
            assert_eq!(
                sigs.len(),
                old_count + delta_count,
                "Discrepancy, the same signature existed "
            );
        } else {
            self.transactions.insert(intent_hash, signatures);
        }
        self.validate();
    }

    /// Returns all the signatures for all the transactions.
    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.transactions
            .values()
            .flat_map(|v| v.iter())
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = MaybeSignedTransactions;

    #[test]
    fn equality_of_samples() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality_of_samples() {
        assert_ne!(Sut::sample(), Sut::sample_other());
    }
}
