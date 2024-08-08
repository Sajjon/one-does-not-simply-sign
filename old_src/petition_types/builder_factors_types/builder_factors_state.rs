use std::cell::Ref;

use super::*;
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BuilderFactorsState {
    /// Factors that have signed.
    signed: RefCell<BuilderFactorsStateSubstate<HDSignature>>,
    /// Factors that user skipped.
    skipped: RefCell<BuilderFactorsStateSubstate<FactorInstance>>,
}

impl BuilderFactorsState {
    pub(super) fn skipped(&self) -> Ref<BuilderFactorsStateSubstate<FactorInstance>> {
        self.skipped.borrow()
    }
    pub(super) fn signed(&self) -> Ref<BuilderFactorsStateSubstate<HDSignature>> {
        self.signed.borrow()
    }

    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.signed().snapshot()
    }

    pub fn all_skipped(&self) -> IndexSet<FactorInstance> {
        self.skipped().snapshot()
    }

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

    pub(crate) fn add_signature(&self, signature: &HDSignature) {
        self.assert_not_referencing_factor_source(signature.factor_source_id());
        self.signed.borrow_mut().insert(signature)
    }

    pub(super) fn new() -> Self {
        Self {
            signed: RefCell::new(BuilderFactorsStateSubstate::<_>::new()),
            skipped: RefCell::new(BuilderFactorsStateSubstate::<_>::new()),
        }
    }

    pub(super) fn snapshot(&self) -> BuilderFactorsStateSnapshot {
        BuilderFactorsStateSnapshot::new(self.signed().snapshot(), self.skipped().snapshot())
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
