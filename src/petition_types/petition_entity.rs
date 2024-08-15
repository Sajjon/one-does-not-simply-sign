use std::cell::Ref;

use crate::prelude::*;

/// Petition of signatures from an entity in a transaction.
/// Essentially a wrapper around a tuple
/// `{ threshold: PetitionFactors, override: PetitionFactors }`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PetitionEntity {
    /// The owner of these factors
    pub entity: AccountAddressOrIdentityAddress,

    /// Index and hash of transaction
    pub intent_hash: IntentHash,

    pub threshold_factors: Option<RefCell<PetitionFactors>>,

    pub override_factors: Option<RefCell<PetitionFactors>>,
}

impl PetitionEntity {
    pub fn new(
        intent_hash: IntentHash,
        entity: AccountAddressOrIdentityAddress,
        threshold_factors: impl Into<Option<PetitionFactors>>,
        override_factors: impl Into<Option<PetitionFactors>>,
    ) -> Self {
        let threshold_factors = threshold_factors.into();
        let override_factors = override_factors.into();
        if threshold_factors.is_none() && override_factors.is_none() {
            panic!("Programmer error! Must have at least one factors list.");
        }
        Self {
            entity,
            intent_hash,
            threshold_factors: threshold_factors.map(RefCell::new),
            override_factors: override_factors.map(RefCell::new),
        }
    }
    pub fn new_securified(
        intent_hash: IntentHash,
        entity: AccountAddressOrIdentityAddress,
        matrix: MatrixOfFactorInstances,
    ) -> Self {
        Self::new(
            intent_hash,
            entity,
            PetitionFactors::new_threshold(matrix.threshold_factors, matrix.threshold as i8),
            PetitionFactors::new_override(matrix.override_factors),
        )
    }
    pub fn new_unsecurified(
        intent_hash: IntentHash,
        entity: AccountAddressOrIdentityAddress,
        instance: FactorInstance,
    ) -> Self {
        Self::new(
            intent_hash,
            entity,
            PetitionFactors::new_unsecurified(instance),
            None,
        )
    }

    pub fn all_factor_instances(&self) -> IndexSet<OwnedFactorInstance> {
        let o: IndexSet<FactorInstance> = self
            .override_factors
            .as_ref()
            .map(|f| f.borrow().factor_instances())
            .unwrap_or_default();

        let t: IndexSet<FactorInstance> = self
            .threshold_factors
            .as_ref()
            .map(|f| f.borrow().factor_instances())
            .unwrap_or_default();

        o.union(&t)
            .map(|f| OwnedFactorInstance::owned_factor_instance(self.entity.clone(), f.clone()))
            .collect::<IndexSet<_>>()
    }

    /// Returns `true` signatures requirement has been fulfilled, either by
    /// override factors or by threshold factors
    pub fn has_signatures_requirement_been_fulfilled(&self) -> bool {
        self.status() == PetitionFactorsStatus::Finished(PetitionFactorsStatusFinished::Success)
    }

    fn union_of<F, T>(&self, map: F) -> IndexSet<T>
    where
        T: Eq + std::hash::Hash + Clone,
        F: Fn(Ref<PetitionFactors>) -> IndexSet<T>,
    {
        let o = self
            .override_factors
            .as_ref()
            .map(|f| map(f.borrow()))
            .unwrap_or_default();

        let t = self
            .threshold_factors
            .as_ref()
            .map(|f| map(f.borrow()))
            .unwrap_or_default();

        o.union(&t).cloned().collect::<IndexSet<T>>()
    }

    pub fn all_skipped_factor_instance(&self) -> IndexSet<FactorInstance> {
        self.union_of(|f| f.all_skipped())
    }

    pub fn all_skipped_factor_sources(&self) -> IndexSet<FactorSourceID> {
        self.all_skipped_factor_instance()
            .into_iter()
            .map(|f| f.factor_source_id)
            .collect::<IndexSet<_>>()
    }

    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        self.union_of(|f| f.all_signatures())
    }

    pub fn skipped_factor_source_if_relevant(&self, factor_source_id: &FactorSourceID) {
        if let Some(t) = self.threshold_factors.as_ref() {
            if t.borrow()
                .references_factor_source_with_id(factor_source_id)
            {
                t.borrow_mut().did_skip(factor_source_id, true);
            }
        }

        if let Some(o) = self.override_factors.as_ref() {
            if o.borrow()
                .references_factor_source_with_id(factor_source_id)
            {
                o.borrow_mut().did_skip(factor_source_id, true);
            }
        }
    }

    /// # Panics
    /// Panics if this factor source has already been skipped or signed with.
    ///
    /// Or panics if the factor source is not known to this petition.
    pub fn add_signature(&self, signature: HDSignature) {
        let mut added_to_threshold = false;
        let mut added_to_override = false;

        if let Some(t) = self.threshold_factors.as_ref() {
            let has = t
                .borrow()
                .has_instance_with_id(signature.owned_factor_instance());
            if has {
                t.borrow_mut().add_signature(&signature);
                added_to_threshold = true;
            }
        }

        if let Some(o) = self.override_factors.as_ref() {
            let has = o
                .borrow()
                .has_instance_with_id(signature.owned_factor_instance());
            if has {
                o.borrow_mut().add_signature(&signature);
                added_to_override = true;
            }
        }

        if added_to_override && added_to_threshold {
            panic!("A factor source should only be present in one of the lists.");
        } else if !added_to_override && !added_to_threshold {
            panic!("Factor source not found in any of the lists.");
        }
    }

    pub fn invalid_transactions_if_skipped(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        let skip_status = self.status_if_skipped_factor_source(factor_source_id);
        match skip_status {
            PetitionFactorsStatus::Finished(finished_reason) => match finished_reason {
                PetitionFactorsStatusFinished::Fail => {
                    let intent_hash = self.intent_hash.clone();
                    let invalid_transaction =
                        InvalidTransactionIfSkipped::new(intent_hash, vec![self.entity.clone()]);
                    IndexSet::from_iter([invalid_transaction])
                }
                PetitionFactorsStatusFinished::Success => IndexSet::new(),
            },
            PetitionFactorsStatus::InProgress => IndexSet::new(),
        }
    }

    /// `Ok(true)` means "continue", `Ok(false)` means "stop, we are done". `Err(_)` means "stop, we have failed".
    pub(super) fn continue_if_necessary(&self) -> Result<bool> {
        match self.status() {
            PetitionFactorsStatus::InProgress => Ok(true),
            PetitionFactorsStatus::Finished(PetitionFactorsStatusFinished::Fail) => {
                Err(CommonError::Failure)
            }
            PetitionFactorsStatus::Finished(PetitionFactorsStatusFinished::Success) => Ok(false),
        }
    }

    fn petition(&self, factor_source_id: &FactorSourceID) -> Option<FactorListKind> {
        if let Some(t) = self.threshold_factors.as_ref() {
            if t.borrow()
                .references_factor_source_with_id(factor_source_id)
            {
                return Some(FactorListKind::Threshold);
            }
        }

        if let Some(o) = self.override_factors.as_ref() {
            if o.borrow()
                .references_factor_source_with_id(factor_source_id)
            {
                return Some(FactorListKind::Override);
            }
        }

        None
    }

    pub fn status_if_skipped_factor_source(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> PetitionFactorsStatus {
        let simulation = self.clone();
        simulation.did_skip(factor_source_id, true);
        simulation.status()
    }

    pub fn did_skip(&self, factor_source_id: &FactorSourceID, simulated: bool) {
        let Some(petition) = self.petition(factor_source_id) else {
            return;
        };
        match petition {
            FactorListKind::Threshold => self
                .threshold_factors
                .as_ref()
                .expect("Should have threshold factors!")
                .borrow_mut()
                .did_skip(factor_source_id, simulated),
            FactorListKind::Override => self
                .override_factors
                .as_ref()
                .expect("Should have override factors!")
                .borrow_mut()
                .did_skip(factor_source_id, simulated),
        }
    }

    pub fn status(&self) -> PetitionFactorsStatus {
        use PetitionFactorsStatus::*;
        use PetitionFactorsStatusFinished::*;

        let maybe_threshold = self.threshold_factors.as_ref().map(|t| t.borrow().status());
        let maybe_override = self.override_factors.as_ref().map(|o| o.borrow().status());

        match (maybe_threshold, maybe_override) {
            (None, None) => panic!("Programmer error! Should have at least one factors list."),
            (Some(threshold), None) => threshold,
            (None, Some(r#override)) => r#override,
            (Some(threshold), Some(r#override)) => match (threshold, r#override) {
                (InProgress, InProgress) => PetitionFactorsStatus::InProgress,
                (Finished(Fail), InProgress) => PetitionFactorsStatus::InProgress,
                (InProgress, Finished(Fail)) => PetitionFactorsStatus::InProgress,
                (Finished(Fail), Finished(Fail)) => PetitionFactorsStatus::Finished(Fail),
                (Finished(Success), _) => PetitionFactorsStatus::Finished(Success),
                (_, Finished(Success)) => PetitionFactorsStatus::Finished(Success),
            },
        }
    }
}

impl PetitionEntity {
    fn from_entity(entity: Entity, intent_hash: IntentHash) -> Self {
        match entity.security_state {
            EntitySecurityState::Securified(matrix) => {
                Self::new_securified(intent_hash, entity.address, matrix)
            }
            EntitySecurityState::Unsecured(factor) => {
                Self::new_unsecurified(intent_hash, entity.address, factor)
            }
        }
    }
}
impl HasSampleValues for PetitionEntity {
    fn sample() -> Self {
        Self::from_entity(Entity::sample_securified(), IntentHash::sample())
    }
    fn sample_other() -> Self {
        Self::from_entity(Entity::sample_unsecurified(), IntentHash::sample_other())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = PetitionEntity;

    #[test]
    #[should_panic(expected = "Programmer error! Must have at least one factors list.")]
    fn invalid_empty_factors() {
        Sut::new(
            IntentHash::sample(),
            AccountAddressOrIdentityAddress::sample(),
            None,
            None,
        );
    }

    #[test]
    #[should_panic(expected = "Factor source not found in any of the lists.")]
    fn cannot_add_unrelated_signature() {
        let sut = Sut::sample();
        sut.add_signature(HDSignature::sample());
    }

    #[test]
    #[should_panic(expected = "A factor MUST NOT be present in both threshold AND override list.")]
    fn factor_should_not_be_used_in_both_lists() {
        Entity::securified(0, "Jane Doe", |idx| {
            let fi = FactorInstance::f(idx);
            MatrixOfFactorInstances::new(
                [FactorSourceID::fs0()].map(&fi),
                1,
                [FactorSourceID::fs0()].map(&fi),
            )
        });
    }

    #[test]
    #[should_panic]
    fn cannot_add_same_signature_twice() {
        let intent_hash = IntentHash::sample();
        let entity = Entity::securified(0, "Jane Doe", |idx| {
            let fi = FactorInstance::f(idx);
            MatrixOfFactorInstances::new(
                [FactorSourceID::fs0()].map(&fi),
                1,
                [FactorSourceID::fs1()].map(&fi),
            )
        });
        let sut = Sut::from_entity(entity.clone(), intent_hash.clone());
        let sign_input = HDSignatureInput::new(
            intent_hash,
            OwnedFactorInstance::new(
                entity.address.clone(),
                FactorInstance::new(0, FactorSourceID::fs0()),
            ),
        );
        let signature = HDSignature::produced_signing_with_input(sign_input);

        sut.add_signature(signature.clone());
        sut.add_signature(signature.clone());
    }
}
