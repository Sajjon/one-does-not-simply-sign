use crate::prelude::*;

/// Some value with a known owner - an account or persona.
#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{:?}: {:?}", owner, value)]
pub struct Owned<T> {
    /// The known owner - an account or persona - of `value`.
    pub owner: AccountAddressOrIdentityAddress,
    /// Some value known to be owned by `owner` - an account or persona.
    pub value: T,
}

impl<T> Owned<T> {
    pub fn new(owner: AccountAddressOrIdentityAddress, value: T) -> Self {
        Self { owner, value }
    }
}

impl<T: HasSampleValues> HasSampleValues for Owned<T> {
    fn sample() -> Self {
        Self::new(AccountAddressOrIdentityAddress::sample(), T::sample())
    }
    fn sample_other() -> Self {
        Self::new(
            AccountAddressOrIdentityAddress::sample_other(),
            T::sample_other(),
        )
    }
}
