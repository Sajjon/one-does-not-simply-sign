use std::borrow::Borrow;

use crate::prelude::*;

/// A `FactorInstance` with a known owner - an account or persona.
pub type OwnedFactorInstance = Owned<FactorInstance>;

impl OwnedFactorInstance {
    /// Constructs a new `OwnedFactorInstance`.
    pub fn owned_factor_instance(
        owner: AccountAddressOrIdentityAddress,
        factor_instance: FactorInstance,
    ) -> Self {
        Self::new(owner, factor_instance)
    }

    /// The owned `FactorInstance`, the value of this `OwnedFactorInstance`.
    pub fn factor_instance(&self) -> &FactorInstance {
        &self.value
    }

    /// Checks if this `OwnedFactorInstance` was created from the factor source
    /// with id `factor_source_id`.
    pub fn by_factor_source(&self, factor_source_id: impl Borrow<FactorSourceID>) -> bool {
        let factor_source_id = factor_source_id.borrow();
        self.factor_instance().factor_source_id() == *factor_source_id
    }
}
