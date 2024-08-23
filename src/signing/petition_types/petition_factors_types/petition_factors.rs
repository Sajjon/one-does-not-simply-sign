use super::*;
use crate::prelude::*;

/// Petition of signatures from a factors list of an entity in a transaction.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub struct PetitionFactors {
    pub factor_list_kind: FactorListKind,

    /// Factors to sign with and the required number of them.
    pub(crate) input: PetitionFactorsInput,
    state: RefCell<PetitionFactorsState>,
}

impl PetitionFactors {
    pub fn debug_str(&self) -> String {
        format!(
            "PetitionFactors(input: {:#?}, state_snapshot: {:#?})",
            self.input,
            self.state_snapshot()
        )
    }
    pub fn new(factor_list_kind: FactorListKind, input: PetitionFactorsInput) -> Self {
        Self {
            factor_list_kind,
            input,
            state: RefCell::new(PetitionFactorsState::new()),
        }
    }

    pub fn factor_instances(&self) -> IndexSet<HierarchicalDeterministicFactorInstance> {
        self.input.factors.clone()
    }

    pub fn all_skipped(&self) -> IndexSet<HierarchicalDeterministicFactorInstance> {
        self.state.borrow().all_skipped()
    }

    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.state.borrow().all_signatures()
    }

    pub fn new_threshold(
        factors: Vec<HierarchicalDeterministicFactorInstance>,
        threshold: i8,
    ) -> Option<Self> {
        if factors.is_empty() {
            return None;
        }
        Some(Self::new(
            FactorListKind::Threshold,
            PetitionFactorsInput::new_threshold(IndexSet::from_iter(factors), threshold),
        ))
    }

    pub fn new_unsecurified(factor: HierarchicalDeterministicFactorInstance) -> Self {
        Self::new_threshold(vec![factor], 1).unwrap() // define as 1/1 threshold factor, which is a good definition.
    }

    pub fn new_override(factors: Vec<HierarchicalDeterministicFactorInstance>) -> Option<Self> {
        if factors.is_empty() {
            return None;
        }
        Some(Self::new(
            FactorListKind::Override,
            PetitionFactorsInput::new_override(IndexSet::from_iter(factors)),
        ))
    }

    pub fn did_skip_if_relevant(&self, factor_source_id: &FactorSourceID, simulated: bool) {
        if let Some(_x_) = self.reference_to_factor_source_with_id(factor_source_id) {
            self.did_skip(factor_source_id, simulated)
        }
    }

    fn did_skip(&self, factor_source_id: &FactorSourceID, simulated: bool) {
        let factor_instance = self.expect_reference_to_factor_source_with_id(factor_source_id);
        self.state.borrow_mut().did_skip(factor_instance, simulated);
    }

    pub fn has_owned_instance_with_id(&self, owned_factor_instance: &OwnedFactorInstance) -> bool {
        self.has_instance_with_id(owned_factor_instance.factor_instance())
    }

    pub fn has_instance_with_id(
        &self,
        factor_instance: &HierarchicalDeterministicFactorInstance,
    ) -> bool {
        self.input.factors.iter().any(|f| f == factor_instance)
    }

    pub fn add_signature_if_relevant(&self, signature: &HDSignature) -> bool {
        if self.has_owned_instance_with_id(signature.owned_factor_instance()) {
            self.add_signature(signature);
            true
        } else {
            false
        }
    }

    /// # Panics
    /// Panics if this factor source has already been skipped or signed with.
    fn add_signature(&self, signature: &HDSignature) {
        let state = self.state.borrow_mut();
        state.add_signature(signature)
    }

    pub fn references_factor_source_with_id(&self, factor_source_id: &FactorSourceID) -> bool {
        self.reference_to_factor_source_with_id(factor_source_id)
            .is_some()
    }

    pub fn skip_if_references(&self, factor_source_id: &FactorSourceID, simulated: bool) {
        if self.references_factor_source_with_id(factor_source_id) {
            self.did_skip(factor_source_id, simulated)
        }
    }

    fn expect_reference_to_factor_source_with_id(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> &HierarchicalDeterministicFactorInstance {
        self.reference_to_factor_source_with_id(factor_source_id)
            .expect("Programmer error! Factor source not found in factors.")
    }

    fn reference_to_factor_source_with_id(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> Option<&HierarchicalDeterministicFactorInstance> {
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
