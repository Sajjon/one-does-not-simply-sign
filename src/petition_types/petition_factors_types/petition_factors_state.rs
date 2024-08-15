use std::cell::Ref;

use super::*;
use crate::prelude::*;

/// Mutable state of `PetitionFactors`, keeping track of which factors that
/// have either signed or been skipped.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PetitionFactorsState {
    /// Factors that have signed.
    signed: RefCell<PetitionFactorsSubState<HDSignature>>,

    /// Factors that user skipped.
    skipped: RefCell<PetitionFactorsSubState<FactorInstance>>,
}

impl PetitionFactorsState {
    pub(super) fn skipped(&self) -> Ref<PetitionFactorsSubState<FactorInstance>> {
        self.skipped.borrow()
    }
    pub(super) fn signed(&self) -> Ref<PetitionFactorsSubState<HDSignature>> {
        self.signed.borrow()
    }

    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.signed().snapshot()
    }

    pub fn all_skipped(&self) -> IndexSet<FactorInstance> {
        self.skipped().snapshot()
    }

    /// # Panics
    /// Panics if this factor source has already been skipped or signed with.
    fn assert_not_referencing_factor_source(&self, factor_source_id: FactorSourceID) {
        if self.references_factor_source_by_id(factor_source_id) {
            panic!("Programmer error! Factor source {:?} already used, should only be referenced once.", factor_source_id);
        }
    }

    pub(crate) fn did_skip(&self, factor_instance: &FactorInstance, simulated: bool) {
        if !simulated {
            self.assert_not_referencing_factor_source(factor_instance.factor_source_id);
        }
        self.skipped.borrow_mut().insert(factor_instance);
    }

    /// # Panics
    /// Panics if this factor source has already been skipped or signed with.
    pub(crate) fn add_signature(&self, signature: &HDSignature) {
        self.assert_not_referencing_factor_source(signature.factor_source_id());
        self.signed.borrow_mut().insert(signature)
    }

    pub(super) fn new() -> Self {
        Self {
            signed: RefCell::new(PetitionFactorsSubState::<_>::new()),
            skipped: RefCell::new(PetitionFactorsSubState::<_>::new()),
        }
    }

    pub(super) fn snapshot(&self) -> PetitionFactorsStateSnapshot {
        PetitionFactorsStateSnapshot::new(self.signed().snapshot(), self.skipped().snapshot())
    }

    fn references_factor_source_by_id(&self, factor_source_id: FactorSourceID) -> bool {
        if self
            .signed
            .borrow()
            .references_factor_source_by_id(factor_source_id)
        {
            return true;
        }

        if self
            .skipped
            .borrow()
            .references_factor_source_by_id(factor_source_id)
        {
            return true;
        }

        false
    }
}
