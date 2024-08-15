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
    /// Creates a new `PetitionFactorsState`.
    pub(super) fn new() -> Self {
        Self {
            signed: RefCell::new(PetitionFactorsSubState::<_>::new()),
            skipped: RefCell::new(PetitionFactorsSubState::<_>::new()),
        }
    }

    /// A reference to the skipped factors so far.
    pub(super) fn skipped(&self) -> Ref<PetitionFactorsSubState<FactorInstance>> {
        self.skipped.borrow()
    }

    /// A reference to the factors which have been signed with so far.
    pub(super) fn signed(&self) -> Ref<PetitionFactorsSubState<HDSignature>> {
        self.signed.borrow()
    }

    /// A set of signatures from factors that have been signed with so far.
    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.signed().snapshot()
    }

    /// A set factors have been skipped so far.
    pub fn all_skipped(&self) -> IndexSet<FactorInstance> {
        self.skipped().snapshot()
    }

    /// # Panics
    /// Panics if this factor source has already been skipped or signed with.
    fn assert_not_referencing_factor_source(&self, factor_source_id: FactorSourceID) {
        assert!(
            !self.references_factor_source_by_id(factor_source_id),
            "Programmer error! Factor source {:?} already used, should only be referenced once.",
            factor_source_id,
        );
    }

    /// # Panics
    /// Panics if this factor source has already been skipped or signed and
    /// this is not a simulation.
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

    pub(super) fn snapshot(&self) -> PetitionFactorsStateSnapshot {
        PetitionFactorsStateSnapshot::new(self.signed().snapshot(), self.skipped().snapshot())
    }

    fn references_factor_source_by_id(&self, factor_source_id: FactorSourceID) -> bool {
        self.signed()
            .references_factor_source_by_id(factor_source_id)
            || self
                .skipped()
                .references_factor_source_by_id(factor_source_id)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    type Sut = PetitionFactorsState;

    #[test]
    #[should_panic]
    fn skipping_twice_panics() {
        let sut = Sut::new();
        let fi = FactorInstance::sample();
        sut.did_skip(&fi, false);
        sut.did_skip(&fi, false);
    }

    #[test]
    #[should_panic]
    fn signing_twice_panics() {
        let sut = Sut::new();
        let sig = HDSignature::sample();
        sut.add_signature(&sig);
        sut.add_signature(&sig);
    }

    #[test]
    #[should_panic]
    fn skipping_already_signed_panics() {
        let sut = Sut::new();

        let intent_hash = IntentHash::sample();

        let factor_instance = FactorInstance::new(0, FactorSourceID::fs0());
        let sign_input = HDSignatureInput::new(
            intent_hash,
            OwnedFactorInstance::new(
                AccountAddressOrIdentityAddress::sample(),
                factor_instance.clone(),
            ),
        );
        let signature = HDSignature::produced_signing_with_input(sign_input);

        sut.add_signature(&signature);

        sut.did_skip(&factor_instance, false);
    }

    #[test]
    #[should_panic]
    fn signing_already_skipped_panics() {
        let sut = Sut::new();

        let intent_hash = IntentHash::sample();
        let factor_instance = FactorInstance::new(0, FactorSourceID::fs0());

        sut.did_skip(&factor_instance, false);

        let sign_input = HDSignatureInput::new(
            intent_hash,
            OwnedFactorInstance::new(
                AccountAddressOrIdentityAddress::sample(),
                factor_instance.clone(),
            ),
        );

        let signature = HDSignature::produced_signing_with_input(sign_input);
        sut.add_signature(&signature);
    }
}
