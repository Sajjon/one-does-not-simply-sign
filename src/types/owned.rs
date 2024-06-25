use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{:?}: {:?}", owner, value)]
pub struct Owned<T> {
    pub owner: AccountAddressOrIdentityAddress,
    pub value: T,
}

impl<T> Owned<T> {
    pub fn new(owner: AccountAddressOrIdentityAddress, value: T) -> Self {
        Self { owner, value }
    }
}
