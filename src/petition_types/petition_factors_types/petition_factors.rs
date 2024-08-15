use super::*;
use crate::prelude::*;

/// Petition of signatures from a factors list of an entity in a transaction.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PetitionFactors {
    factor_list_kind: FactorListKind,

    /// Factors to sign with and the required number of them.
    input: PetitionFactorsInput,
    state: RefCell<PetitionFactorsState>,
}

impl PetitionFactors {
    pub fn new(factor_list_kind: FactorListKind, input: PetitionFactorsInput) -> Self {
        Self {
            factor_list_kind,
            input,
            state: RefCell::new(PetitionFactorsState::new()),
        }
    }

    pub fn factor_instances(&self) -> IndexSet<FactorInstance> {
        self.input.factors.clone()
    }

    pub fn all_skipped(&self) -> IndexSet<FactorInstance> {
        self.state.borrow().all_skipped()
    }

    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.state.borrow().all_signatures()
    }

    pub fn new_threshold(factors: Vec<FactorInstance>, threshold: i8) -> Option<Self> {
        if factors.is_empty() {
            return None;
        }
        Some(Self::new(
            FactorListKind::Threshold,
            PetitionFactorsInput::new_threshold(IndexSet::from_iter(factors), threshold),
        ))
    }

    pub fn new_unsecurified(factor: FactorInstance) -> Self {
        Self::new_threshold(vec![factor], 1).unwrap() // define as 1/1 threshold factor, which is a good definition.
    }

    pub fn new_override(factors: Vec<FactorInstance>) -> Option<Self> {
        if factors.is_empty() {
            return None;
        }
        Some(Self::new(
            FactorListKind::Override,
            PetitionFactorsInput::new_override(IndexSet::from_iter(factors)),
        ))
    }

    pub fn new_not_used() -> Self {
        Self {
            factor_list_kind: FactorListKind::Override, // does not matter..
            input: PetitionFactorsInput {
                factors: IndexSet::new(),
                required: 0,
            },
            state: RefCell::new(PetitionFactorsState::new()),
        }
    }

    pub fn did_skip(&self, factor_source_id: &FactorSourceID, simulated: bool) {
        let factor_instance = self.expect_reference_to_factor_source_with_id(factor_source_id);
        self.state.borrow_mut().did_skip(factor_instance, simulated);
    }

    pub fn has_instance_with_id(&self, owned_factor_instance: &OwnedFactorInstance) -> bool {
        self.input
            .factors
            .iter()
            .any(|f| f == owned_factor_instance.factor_instance())
    }

    /// # Panics
    /// Panics if this factor source has already been skipped or signed with.
    pub fn add_signature(&self, signature: &HDSignature) {
        let state = self.state.borrow_mut();
        state.add_signature(signature)
    }

    pub fn references_factor_source_with_id(&self, factor_source_id: &FactorSourceID) -> bool {
        self.reference_to_factor_source_with_id(factor_source_id)
            .is_some()
    }

    fn expect_reference_to_factor_source_with_id(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> &FactorInstance {
        self.reference_to_factor_source_with_id(factor_source_id)
            .expect("Programmer error! Factor source not found in factors.")
    }

    fn reference_to_factor_source_with_id(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> Option<&FactorInstance> {
        self.input.reference_factor_source_with_id(factor_source_id)
    }

    fn state_snapshot(&self) -> PetitionFactorsStateSnapshot {
        self.state.borrow().snapshot()
    }

    fn is_finished_successfully(&self) -> bool {
        self.input.is_fulfilled_by(self.state_snapshot())
    }

    fn is_finished_with_fail(&self) -> bool {
        self.input.is_failure_with(self.state_snapshot())
    }

    fn finished_with(&self) -> Option<PetitionFactorsStatusFinished> {
        if self.is_finished_successfully() {
            Some(PetitionFactorsStatusFinished::Success)
        } else if self.is_finished_with_fail() {
            Some(PetitionFactorsStatusFinished::Fail)
        } else {
            None
        }
    }

    pub fn status(&self) -> PetitionFactorsStatus {
        if let Some(finished_state) = self.finished_with() {
            return PetitionFactorsStatus::Finished(finished_state);
        }
        PetitionFactorsStatus::InProgress
    }
}
