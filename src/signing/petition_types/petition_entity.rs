use crate::prelude::*;

/// Petition of signatures from an entity in a transaction.
/// Essentially a wrapper around a tuple
/// `{ threshold: PetitionFactors, override: PetitionFactors }`
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub struct PetitionEntity {
    /// The owner of these factors
    pub entity: AddressOfAccountOrPersona,

    /// Index and hash of transaction
    pub intent_hash: IntentHash,

    /// Petition with threshold factors
    pub threshold_factors: Option<RefCell<PetitionFactors>>,

    /// Petition with override factors
    pub override_factors: Option<RefCell<PetitionFactors>>,
}

impl PetitionEntity {
    #[allow(unused)]
    fn debug_str(&self) -> String {
        let thres: String = self
            .threshold_factors
            .clone()
            .map(|f| format!("threshold_factors {:#?}", f.borrow()))
            .unwrap_or_default();

        let overr: String = self
            .override_factors
            .clone()
            .map(|f| format!("override_factors {:#?}", f.borrow()))
            .unwrap_or_default();

        format!(
            "intent_hash: {:#?}, entity: {:#?}, {:#?}{:#?}",
            self.intent_hash, self.entity, thres, overr
        )
    }

    pub fn new(
        intent_hash: IntentHash,
        entity: AddressOfAccountOrPersona,
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
        entity: AddressOfAccountOrPersona,
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
        entity: AddressOfAccountOrPersona,
        instance: HierarchicalDeterministicFactorInstance,
    ) -> Self {
        Self::new(
            intent_hash,
            entity,
            PetitionFactors::new_unsecurified(instance),
            None,
        )
    }

    /// Returns `true` signatures requirement has been fulfilled, either by
    /// override factors or by threshold factors
    pub fn has_signatures_requirement_been_fulfilled(&self) -> bool {
        self.status() == PetitionFactorsStatus::Finished(PetitionFactorsStatusFinished::Success)
    }

    fn union_of<F, T>(&self, map: F) -> IndexSet<T>
    where
        T: Eq + std::hash::Hash + Clone,
        F: Fn(&PetitionFactors) -> IndexSet<T>,
    {
        self.both(
            |l| map(l),
            |t, o| {
                t.unwrap_or_default()
                    .union(&o.unwrap_or_default())
                    .cloned()
                    .collect::<IndexSet<T>>()
            },
        )
    }

    pub fn all_factor_instances(&self) -> IndexSet<OwnedFactorInstance> {
        self.union_of(|l| l.factor_instances())
            .into_iter()
            .map(|f| OwnedFactorInstance::owned_factor_instance(self.entity.clone(), f.clone()))
            .collect::<IndexSet<_>>()
    }

    pub fn all_skipped_factor_instance(&self) -> IndexSet<HierarchicalDeterministicFactorInstance> {
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

    fn with_list<F, T>(list: &Option<RefCell<PetitionFactors>>, map: F) -> Option<T>
    where
        F: Fn(&PetitionFactors) -> T,
    {
        list.as_ref().map(|refcell| map(&refcell.borrow()))
    }

    fn on_list<F, R>(&self, kind: FactorListKind, r#do: &F) -> Option<R>
    where
        F: Fn(&PetitionFactors) -> R,
    {
        match kind {
            FactorListKind::Threshold => Self::with_list(&self.threshold_factors, r#do),
            FactorListKind::Override => Self::with_list(&self.override_factors, r#do),
        }
    }

    fn both<F, C, T, R>(&self, r#do: F, combine: C) -> R
    where
        F: Fn(&PetitionFactors) -> T,
        C: Fn(Option<T>, Option<T>) -> R,
    {
        let t = self.on_list(FactorListKind::Threshold, &r#do);
        let o = self.on_list(FactorListKind::Override, &r#do);
        combine(t, o)
    }

    fn both_void<F, R>(&self, r#do: F)
    where
        F: Fn(&PetitionFactors) -> R,
    {
        self.both(r#do, |_, _| ())
    }

    pub fn skipped_factor_source_if_relevant(&self, factor_source_id: &FactorSourceID) {
        self.both_void(|l| l.skip_if_references(factor_source_id, true));
    }

    /// # Panics
    /// Panics if this factor source has already been skipped or signed with.
    ///
    /// Or panics if the factor source is not known to this petition.
    pub fn add_signature(&self, signature: HDSignature) {
        self.both(|l| l.add_signature_if_relevant(&signature), |t, o| {
            match (t, o) {
                (Some(true), Some(true)) => {
                    unreachable!("Matrix of FactorInstances does not allow for a factor to be present in both threshold and override list, thus this will never happen.")
                }
                (Some(false), Some(false)) => panic!("Factor source not found in any of the lists."),
                (None, None) => panic!("Programmer error! Must have at least one factors list."), 
                _ => (),
            }
        })
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

    pub fn status_if_skipped_factor_source(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> PetitionFactorsStatus {
        let simulation = self.clone();
        simulation
            .did_skip_if_relevant(factor_source_id, true)
            .unwrap();
        simulation.status()
    }

    pub fn did_skip_if_relevant(
        &self,
        factor_source_id: &FactorSourceID,
        simulated: bool,
    ) -> Result<()> {
        self.both_void(|l| l.did_skip_if_relevant(factor_source_id, simulated));
        Ok(())
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
    fn from_entity(entity: impl Into<AccountOrPersona>, intent_hash: IntentHash) -> Self {
        let entity = entity.into();
        match entity.security_state() {
            EntitySecurityState::Securified(matrix) => {
                Self::new_securified(intent_hash, entity.address(), matrix)
            }
            EntitySecurityState::Unsecured(factor) => {
                Self::new_unsecurified(intent_hash, entity.address(), factor)
            }
        }
    }
}

impl HasSampleValues for PetitionEntity {
    fn sample() -> Self {
        Self::from_entity(Account::sample_securified(), IntentHash::sample())
    }

    fn sample_other() -> Self {
        Self::from_entity(Account::sample_unsecurified(), IntentHash::sample_other())
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
            AddressOfAccountOrPersona::sample(),
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
        Account::securified_mainnet(0, "Jane Doe", |idx| {
            let fi = HierarchicalDeterministicFactorInstance::f(CAP26EntityKind::Account, idx);
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
        let entity = Account::securified_mainnet(0, "Jane Doe", |idx| {
            let fi = HierarchicalDeterministicFactorInstance::f(CAP26EntityKind::Account, idx);
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
                entity.address(),
                HierarchicalDeterministicFactorInstance::mainnet_tx_account(
                    0,
                    FactorSourceID::fs0(),
                ),
            ),
        );
        let signature = HDSignature::produced_signing_with_input(sign_input);

        sut.add_signature(signature.clone());
        sut.add_signature(signature.clone());
    }

    #[test]
    fn invalid_transactions_if_skipped_success() {
        let sut = Sut::sample();
        sut.add_signature(HDSignature::produced_signing_with_input(
            HDSignatureInput::new(
                sut.intent_hash.clone(),
                OwnedFactorInstance::new(
                    sut.entity.clone(),
                    HierarchicalDeterministicFactorInstance::mainnet_tx_account(
                        6,
                        FactorSourceID::fs1(),
                    ),
                ),
            ),
        ));
        let can_skip = |f: FactorSourceID| {
            assert!(sut
                // Already signed with override factor `FactorSourceID::fs1()`. Thus
                // can skip
                .invalid_transactions_if_skipped(&f)
                .is_empty())
        };
        can_skip(FactorSourceID::fs0());
        can_skip(FactorSourceID::fs3());
        can_skip(FactorSourceID::fs4());
        can_skip(FactorSourceID::fs5());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other())
    }
}
