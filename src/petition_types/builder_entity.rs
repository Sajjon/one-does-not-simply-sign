use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuilderEntity {
    /// The owner of these factors
    pub entity: AccountAddressOrIdentityAddress,

    /// Index and hash of transaction
    pub intent_hash: IntentHash,

    pub threshold_factors: Option<RefCell<BuilderFactors>>,

    pub override_factors: Option<RefCell<BuilderFactors>>,
}

impl BuilderEntity {
    pub fn new(
        intent_hash: IntentHash,
        entity: AccountAddressOrIdentityAddress,
        threshold_factors: impl Into<Option<BuilderFactors>>,
        override_factors: impl Into<Option<BuilderFactors>>,
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
            BuilderFactors::new_threshold(matrix.threshold_factors, matrix.threshold as i8),
            BuilderFactors::new_override(matrix.override_factors),
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
            BuilderFactors::new_unsecurified(instance),
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
        self.status() == BuilderFactorsStatus::Finished(BuilderFactorsStatusFinished::Success)
    }

    pub fn all_signatures(&self) -> IndexSet<HDSignature> {
        let o: IndexSet<HDSignature> = self
            .override_factors
            .as_ref()
            .map(|f| f.borrow().all_signatures())
            .unwrap_or_default();

        let t: IndexSet<HDSignature> = self
            .threshold_factors
            .as_ref()
            .map(|f| f.borrow().all_signatures())
            .unwrap_or_default();

        o.union(&t).map(|x| x.to_owned()).collect::<IndexSet<_>>()
    }

    pub fn references_factor_source_with_id(&self, factor_source_id: &FactorSourceID) -> bool {
        if let Some(references) = self.override_factors.as_ref().map(|o| {
            o.borrow()
                .references_factor_source_with_id(factor_source_id)
        }) {
            return references;
        }

        if let Some(references) = self.threshold_factors.as_ref().map(|t| {
            t.borrow()
                .references_factor_source_with_id(factor_source_id)
        }) {
            return references;
        }

        panic!("Programmer error! Should have at least one factors list.");
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
            BuilderFactorsStatus::Finished(finished_reason) => match finished_reason {
                BuilderFactorsStatusFinished::Fail => {
                    let intent_hash = self.intent_hash.clone();
                    let invalid_transaction =
                        InvalidTransactionIfSkipped::new(intent_hash, vec![self.entity.clone()]);
                    IndexSet::from_iter([invalid_transaction])
                }
                BuilderFactorsStatusFinished::Success => IndexSet::new(),
            },
            BuilderFactorsStatus::InProgress => IndexSet::new(),
        }
    }

    /// `Ok(true)` means "continue", `Ok(false)` means "stop, we are done". `Err(_)` means "stop, we have failed".
    pub(super) fn continue_if_necessary(&self) -> Result<bool> {
        match self.status() {
            BuilderFactorsStatus::InProgress => Ok(true),
            BuilderFactorsStatus::Finished(BuilderFactorsStatusFinished::Fail) => {
                Err(CommonError::Failure)
            }
            BuilderFactorsStatus::Finished(BuilderFactorsStatusFinished::Success) => Ok(false),
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
    ) -> BuilderFactorsStatus {
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

    pub fn status(&self) -> BuilderFactorsStatus {
        use BuilderFactorsStatus::*;
        use BuilderFactorsStatusFinished::*;

        let maybe_threshold = self.threshold_factors.as_ref().map(|t| t.borrow().status());
        let maybe_override = self.override_factors.as_ref().map(|o| o.borrow().status());

        match (maybe_threshold, maybe_override) {
            (None, None) => panic!("Programmer error! Should have at least one factors list."),
            (Some(threshold), None) => threshold,
            (None, Some(r#override)) => r#override,
            (Some(threshold), Some(r#override)) => match (threshold, r#override) {
                (InProgress, InProgress) => BuilderFactorsStatus::InProgress,
                (Finished(Fail), InProgress) => BuilderFactorsStatus::InProgress,
                (InProgress, Finished(Fail)) => BuilderFactorsStatus::InProgress,
                (Finished(Fail), Finished(Fail)) => BuilderFactorsStatus::Finished(Fail),
                (Finished(Success), _) => BuilderFactorsStatus::Finished(Success),
                (_, Finished(Success)) => BuilderFactorsStatus::Finished(Success),
            },
        }
    }
}
