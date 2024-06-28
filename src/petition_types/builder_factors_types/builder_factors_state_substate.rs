use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BuilderFactorsStateSubstate<F>
where
    F: FactorSourceReferencing,
{
    /// Factors that have signed or skipped
    factors: RefCell<IndexSet<F>>,
}

impl<F: FactorSourceReferencing> BuilderFactorsStateSubstate<F> {
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

    pub(super) fn references_factor_source_by_id(&self, factor_source_id: FactorSourceID) -> bool {
        self.factors
            .borrow()
            .iter()
            .any(|sf| sf.factor_source_id() == factor_source_id)
    }
}

pub trait FactorSourceReferencing: std::hash::Hash + PartialEq + Eq + Clone {
    fn factor_source_id(&self) -> FactorSourceID;
}

impl FactorSourceReferencing for FactorInstance {
    fn factor_source_id(&self) -> FactorSourceID {
        self.factor_source_id
    }
}

impl FactorSourceReferencing for HDSignature {
    fn factor_source_id(&self) -> FactorSourceID {
        self.owned_factor_instance()
            .factor_instance()
            .factor_source_id
    }
}
