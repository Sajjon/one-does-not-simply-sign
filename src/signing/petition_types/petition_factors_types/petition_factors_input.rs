use super::*;
use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PetitionFactorsInput {
    /// Factors to sign with.
    pub(super) factors: IndexSet<HierarchicalDeterministicFactorInstance>,

    /// Number of required factors to sign with.
    pub(super) required: i8,
}

impl PetitionFactorsInput {
    pub(super) fn new(
        factors: IndexSet<HierarchicalDeterministicFactorInstance>,
        required: i8,
    ) -> Self {
        Self { factors, required }
    }

    pub(super) fn new_threshold(
        factors: IndexSet<HierarchicalDeterministicFactorInstance>,
        threshold: i8,
    ) -> Self {
        Self::new(factors, threshold)
    }

    pub(super) fn new_override(factors: IndexSet<HierarchicalDeterministicFactorInstance>) -> Self {
        Self::new(factors, 1) // we need just one, anyone, factor for threshold.
    }

    pub fn reference_factor_source_with_id(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> Option<&HierarchicalDeterministicFactorInstance> {
        self.factors
            .iter()
            .find(|f| f.factor_source_id == *factor_source_id)
    }

    fn factors_count(&self) -> i8 {
        self.factors.len() as i8
    }

    fn remaining_factors_until_success(&self, snapshot: PetitionFactorsStateSnapshot) -> i8 {
        self.required - snapshot.signed_count()
    }

    pub(super) fn is_fulfilled_by(&self, snapshot: PetitionFactorsStateSnapshot) -> bool {
        self.remaining_factors_until_success(snapshot) <= 0
    }

    fn factors_left_to_prompt(&self, snapshot: PetitionFactorsStateSnapshot) -> i8 {
        self.factors_count() - snapshot.prompted_count()
    }

    pub(super) fn is_failure_with(&self, snapshot: PetitionFactorsStateSnapshot) -> bool {
        let signed_or_pending =
            self.factors_left_to_prompt(snapshot.clone()) + snapshot.signed_count();
        signed_or_pending < self.required
    }
}
