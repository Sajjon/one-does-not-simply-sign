use crate::prelude::*;

/// A sub-state of `PetitionFactorsState` which can be used to track factors
/// that have signed or skipped.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PetitionFactorsSubState<F>
where
    F: FactorSourceReferencing,
{
    /// Factors that have signed or skipped
    factors: RefCell<IndexSet<F>>,
}

impl<F: FactorSourceReferencing> PetitionFactorsSubState<F> {
    pub(super) fn new() -> Self {
        Self {
            factors: RefCell::new(IndexSet::new()),
        }
    }

    pub(super) fn insert(&self, factor: &F) {
        self.factors.borrow_mut().insert(factor.clone());
    }

    pub(super) fn snapshot(&self) -> IndexSet<F> {
        self.factors.borrow().clone()
    }

    pub(super) fn references_factor_source_by_id(
        &self,
        factor_source_id: FactorSourceIDFromHash,
    ) -> bool {
        self.factors
            .borrow()
            .iter()
            .any(|sf| sf.factor_source_id() == factor_source_id)
    }
}

pub trait FactorSourceReferencing: std::hash::Hash + PartialEq + Eq + Clone {
    fn factor_source_id(&self) -> FactorSourceIDFromHash;
}

impl FactorSourceReferencing for HierarchicalDeterministicFactorInstance {
    fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.factor_source_id
    }
}

impl FactorSourceReferencing for HDSignature {
    fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.owned_factor_instance()
            .factor_instance()
            .factor_source_id
    }
}
