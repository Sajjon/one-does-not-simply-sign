use crate::prelude::*;

/// A list of entities which would fail in a transaction if we would
/// skip signing with a certain factor source
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct InvalidTransactionIfSkipped {
    /// The intent hash of the transaction which would be invalid if we skipped
    /// signing with a certain factor source
    pub intent_hash: IntentHash,

    /// The entities in the transaction which would fail auth.
    entities_which_would_fail_auth: Vec<AccountAddressOrIdentityAddress>,
}

impl InvalidTransactionIfSkipped {
    /// Constructs a new `InvalidTransactionIfSkipped` from an IndexSet of
    /// entities which would fail auth..
    ///
    /// # Panics
    /// Panics if `entities_which_would_fail_auth` is empty.
    pub fn new(
        intent_hash: IntentHash,
        entities_which_would_fail_auth: impl IntoIterator<Item = AccountAddressOrIdentityAddress>,
    ) -> Self {
        let entities_which_would_fail_auth =
            entities_which_would_fail_auth.into_iter().collect_vec();
        let len = entities_which_would_fail_auth.len();
        let entities_which_would_fail_auth = entities_which_would_fail_auth
            .into_iter()
            .collect::<IndexSet<_>>();
        assert!(!entities_which_would_fail_auth.is_empty(), "'entities_which_would_fail_auth' must not be empty, this type is not useful if it is empty.");
        assert!(
            entities_which_would_fail_auth.len() == len,
            "entities_which_would_fail_auth must not contain duplicates."
        );
        Self {
            intent_hash,
            entities_which_would_fail_auth: entities_which_would_fail_auth
                .into_iter()
                .collect_vec(),
        }
    }

    pub fn entities_which_would_fail_auth(&self) -> IndexSet<AccountAddressOrIdentityAddress> {
        IndexSet::from_iter(self.entities_which_would_fail_auth.clone())
    }

    /// Constructs a new `InvalidTransactionIfSkipped` from a single entity
    /// which would fail auth.
    pub fn new_single_entity(
        intent_hash: IntentHash,
        entity_which_would_fail_auth: AccountAddressOrIdentityAddress,
    ) -> Self {
        Self::new(intent_hash, [entity_which_would_fail_auth])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = InvalidTransactionIfSkipped;

    #[test]
    #[should_panic(
        expected = "'entities_which_would_fail_auth' must not be empty, this type is not useful if it is empty."
    )]
    fn panics_if_empty() {
        Sut::new(IntentHash::sample(), IndexSet::new());
    }

    #[test]
    #[should_panic(expected = "entities_which_would_fail_auth must not contain duplicates.")]
    fn panics_if_duplicates() {
        Sut::new(
            IntentHash::sample(),
            [
                AccountAddressOrIdentityAddress::sample(),
                AccountAddressOrIdentityAddress::sample(),
            ],
        );
    }

    #[test]
    fn new() {
        let entities = [
            AccountAddressOrIdentityAddress::sample(),
            AccountAddressOrIdentityAddress::sample_other(),
        ];
        let sut = Sut::new(IntentHash::sample(), entities.clone());
        assert_eq!(
            sut.entities_which_would_fail_auth(),
            IndexSet::<_>::from_iter(entities.into_iter())
        );
    }
}
