use std::borrow::Borrow;

use crate::prelude::*;

pub type OwnedFactorInstance = Owned<FactorInstance>;

impl OwnedFactorInstance {
    pub fn owned_factor_instance(
        owner: AccountAddressOrIdentityAddress,
        factor_instance: FactorInstance,
    ) -> Self {
        Self::new(owner, factor_instance)
    }
    pub fn factor_instance(&self) -> &FactorInstance {
        &self.value
    }
    pub fn by_factor_source(&self, factor_source_id: impl Borrow<FactorSourceID>) -> bool {
        let factor_source_id = factor_source_id.borrow();
        self.factor_instance().factor_source_id() == *factor_source_id
    }
}
