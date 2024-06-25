use crate::prelude::*;

#[derive(Derivative)]
#[derivative(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
pub struct TransactionIndex {
    index: usize,

    #[derivative(Ord = "ignore", PartialOrd = "ignore")]
    intent_hash: IntentHash,
}
impl TransactionIndex {
    pub fn new(index: usize, intent_hash: IntentHash) -> Self {
        Self { index, intent_hash }
    }
}

#[derive(Derivative)]
#[derivative(PartialOrd, Ord, PartialEq, Eq, Hash, Clone, Debug)]
pub struct PetitionOfTransactionByEntity {
    /// The owner of these factors
    #[derivative(Ord = "ignore", PartialOrd = "ignore")]
    pub entity: AccountAddressOrIdentityAddress,

    /// Index and hash of transaction
    pub transaction_index: TransactionIndex,

    #[derivative(Hash = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    pub threshold_factors: RefCell<PetitionWithFactors>,

    #[derivative(Hash = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    pub override_factors: RefCell<PetitionWithFactors>,
}

impl PetitionOfTransactionByEntity {
    pub fn new(
        transaction_index: TransactionIndex,
        entity: AccountAddressOrIdentityAddress,
        threshold_factors: PetitionWithFactors,
        override_factors: PetitionWithFactors,
    ) -> Self {
        Self {
            entity,
            transaction_index,
            threshold_factors: RefCell::new(threshold_factors),
            override_factors: RefCell::new(override_factors),
        }
    }
    pub fn new_securified(
        transaction_index: TransactionIndex,
        entity: AccountAddressOrIdentityAddress,
        matrix: MatrixOfFactorInstances,
    ) -> Self {
        Self::new(
            transaction_index,
            entity,
            PetitionWithFactors::new_threshold(matrix.threshold_factors, matrix.threshold as i8),
            PetitionWithFactors::new_override(matrix.override_factors),
        )
    }
    pub fn new_unsecurified(
        transaction_index: TransactionIndex,
        entity: AccountAddressOrIdentityAddress,
        instance: FactorInstance,
    ) -> Self {
        Self::new(
            transaction_index,
            entity,
            PetitionWithFactors::new_unsecurified(instance),
            PetitionWithFactors::new_not_used(),
        )
    }
    pub fn all_factor_instances(&self) -> IndexSet<OwnedFactorInstance> {
        self.override_factors
            .borrow()
            .factor_instances()
            .union(&self.threshold_factors.borrow().factor_instances())
            .into_iter()
            .map(|f| OwnedFactorInstance::new(f.clone(), self.entity.clone()))
            .collect::<IndexSet<_>>()
    }

    // pub fn process_outcome(
    //     &self,
    //     outcome: &SignWithFactorSourceOrSourcesOutcome,
    //     factor_source: &FactorSource,
    // ) {
    //     self.threshold_factors
    //         .borrow_mut()
    //         .process_outcome(outcome, factor_source);
    //     self.override_factors
    //         .borrow_mut()
    //         .process_outcome(outcome, factor_source);
    // }

    pub fn add_signature(&self, signature: HDSignature) {
        self.threshold_factors
            .borrow_mut()
            .add_signature_if_matching_factor(&signature);
        self.override_factors
            .borrow_mut()
            .add_signature_if_matching_factor(&signature);
    }

    pub fn invalid_transactions_if_skipped(
        &self,
        factor_source: &FactorSource,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        let skip_status = self.status_if_skipped_factor_source(factor_source);
        match skip_status {
            PetitionForFactorListStatus::Finished(finished_reason) => match finished_reason {
                PetitionForFactorListStatusFinished::Fail => {
                    let transaction_index = self.transaction_index.clone();
                    let intent_hash = transaction_index.intent_hash;
                    let invalid_transaction =
                        InvalidTransactionIfSkipped::new(intent_hash, vec![self.entity.clone()]);
                    IndexSet::from_iter([invalid_transaction])
                }
                PetitionForFactorListStatusFinished::Success => IndexSet::new(),
            },
            PetitionForFactorListStatus::InProgress => IndexSet::new(),
        }
    }

    pub(super) fn continue_if_necessary(&self) -> Result<()> {
        match self.status() {
            PetitionForFactorListStatus::InProgress => Ok(()),
            PetitionForFactorListStatus::Finished(PetitionForFactorListStatusFinished::Fail) => {
                Err(CommonError::Failure)
            }
            PetitionForFactorListStatus::Finished(PetitionForFactorListStatusFinished::Success) => {
                Ok(())
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum Petition {
    Threshold,
    Override,
}

impl PetitionOfTransactionByEntity {
    fn petition(&self, factor_source: &FactorSource) -> Option<Petition> {
        if self
            .threshold_factors
            .borrow()
            .references_factor_source(factor_source)
        {
            Some(Petition::Threshold)
        } else if self
            .override_factors
            .borrow()
            .references_factor_source(factor_source)
        {
            Some(Petition::Override)
        } else {
            None
        }
    }
}
impl PetitionOfTransactionByEntity {
    pub fn status_if_skipped_factor_source(
        &self,
        factor_source: &FactorSource,
    ) -> PetitionForFactorListStatus {
        let simulation = self.clone();
        simulation.did_skip(factor_source, false);
        simulation.status()
    }

    pub fn did_skip(&self, factor_source: &FactorSource, should_assert: bool) {
        let Some(petition) = self.petition(factor_source) else {
            return;
        };
        match petition {
            Petition::Threshold => self
                .threshold_factors
                .borrow_mut()
                .did_skip(factor_source, should_assert),
            Petition::Override => self
                .override_factors
                .borrow_mut()
                .did_skip(factor_source, should_assert),
        }
    }

    pub fn status(&self) -> PetitionForFactorListStatus {
        use PetitionForFactorListStatus::*;
        use PetitionForFactorListStatusFinished::*;
        let threshold = self.threshold_factors.borrow().status();
        let r#override = self.override_factors.borrow().status();

        match (threshold, r#override) {
            (InProgress, InProgress) => PetitionForFactorListStatus::InProgress,
            (Finished(Fail), InProgress) => PetitionForFactorListStatus::InProgress,
            (InProgress, Finished(Fail)) => PetitionForFactorListStatus::InProgress,
            (Finished(Fail), Finished(Fail)) => PetitionForFactorListStatus::Finished(Fail),
            (Finished(Success), _) => PetitionForFactorListStatus::Finished(Success),
            (_, Finished(Success)) => PetitionForFactorListStatus::Finished(Success),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PetitionWithFactors {
    /// Factors to sign with and the required number of them.
    input: PetitionWithFactorsInput,
    state: RefCell<PetitionWithFactorsState>,
}

impl PetitionWithFactors {
    pub fn new(input: PetitionWithFactorsInput) -> Self {
        Self {
            input,
            state: RefCell::new(PetitionWithFactorsState::new()),
        }
    }

    pub fn factor_instances(&self) -> IndexSet<FactorInstance> {
        self.input.factors.clone()
    }

    pub fn new_threshold(factors: Vec<FactorInstance>, threshold: i8) -> Self {
        Self::new(PetitionWithFactorsInput::new_threshold(
            IndexSet::from_iter(factors),
            threshold,
        ))
    }

    pub fn new_unsecurified(factor: FactorInstance) -> Self {
        Self::new_threshold(vec![factor], 1) // define as 1/1 threshold factor, which is a good definition.
    }
    pub fn new_override(factors: Vec<FactorInstance>) -> Self {
        Self::new(PetitionWithFactorsInput::new_override(IndexSet::from_iter(
            factors,
        )))
    }
    pub fn new_not_used() -> Self {
        Self {
            input: PetitionWithFactorsInput {
                factors: IndexSet::new(),
                required: 0,
            },
            state: RefCell::new(PetitionWithFactorsState::new()),
        }
    }
}

impl PetitionWithFactors {
    pub fn did_skip(&self, factor_source: &FactorSource, should_assert: bool) {
        let factor_instance = self.expect_reference_to_factor_source(factor_source);
        self.state
            .borrow_mut()
            .did_skip(factor_instance, should_assert);
    }

    pub fn add_signature_if_matching_factor(&self, signature: &HDSignature) {
        let state = self.state.borrow_mut();
        state.add_signature_if_matching_factor(signature)
    }

    // pub fn process_outcome(
    //     &self,
    //     outcome: &SignWithFactorSourceOrSourcesOutcome,
    //     factor_source: &FactorSource,
    // ) {
    //     let state = self.state.borrow_mut();
    //     match outcome {
    //         SignWithFactorSourceOrSourcesOutcome::Signed(signatures) => {
    //             for signature in signatures {
    //                 state.did_sign(&SignatureByFactor::new(
    //                     signature.signature.clone(),
    //                     signature.owned_factor_instance.factor_instance.clone(),
    //                 ));
    //             }
    //         }
    //         SignWithFactorSourceOrSourcesOutcome::Skipped => {
    //             self.did_skip(factor_source, true);
    //         } // SignWithFactorSourceOrSourcesOutcome::Interrupted(_) => {
    //           //     self.did_skip(factor_source, true);
    //           // }
    //     }
    // }

    pub fn references_factor_source(&self, factor_source: &FactorSource) -> bool {
        self.reference_to_factor_source(factor_source).is_some()
    }

    fn expect_reference_to_factor_source(&self, factor_source: &FactorSource) -> &FactorInstance {
        self.reference_to_factor_source(factor_source)
            .expect("Programmer error! Factor source not found in factors.")
    }

    fn reference_to_factor_source(&self, factor_source: &FactorSource) -> Option<&FactorInstance> {
        self.input.reference_factor_source(factor_source)
    }
}
#[derive(Clone)]
struct PetitionWithFactorsStateSnapshot {
    /// Factors that have signed.
    signed: IndexSet<HDSignature>,
    /// Factors that user skipped.
    skipped: IndexSet<FactorInstance>,
}
impl PetitionWithFactorsStateSnapshot {
    fn prompted_count(&self) -> i8 {
        self.signed_count() + self.skipped_count()
    }

    fn signed_count(&self) -> i8 {
        self.signed.len() as i8
    }

    fn skipped_count(&self) -> i8 {
        self.skipped.len() as i8
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
        self.owned_factor_instance.factor_instance.factor_source_id
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct PetitionWithFactorsStateFactors<F>
where
    F: FactorSourceReferencing,
{
    /// Factors that have signed or skipped
    factors: RefCell<IndexSet<F>>,
}
impl<F: FactorSourceReferencing> PetitionWithFactorsStateFactors<F> {
    fn new() -> Self {
        Self {
            factors: RefCell::new(IndexSet::new()),
        }
    }

    fn insert(&self, factor: &F) {
        self.factors.borrow_mut().insert(factor.clone());
    }

    fn snapshot(&self) -> IndexSet<F> {
        self.factors.borrow().clone()
    }

    fn references_factor_source_by_id(&self, factor_source_id: FactorSourceID) -> bool {
        self.factors
            .borrow()
            .iter()
            .any(|sf| sf.factor_source_id() == factor_source_id)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct PetitionWithFactorsState {
    /// Factors that have signed.
    signed: RefCell<PetitionWithFactorsStateFactors<HDSignature>>,
    /// Factors that user skipped.
    skipped: RefCell<PetitionWithFactorsStateFactors<FactorInstance>>,
}

impl PetitionWithFactorsState {
    fn assert_not_referencing_factor_source(&self, factor_source_id: FactorSourceID) {
        assert!(
            self.references_factor_source_by_id(factor_source_id),
            "Programmer error! Factor source already used, should only be referenced once."
        );
    }

    pub(crate) fn did_skip(&self, factor_instance: &FactorInstance, should_assert: bool) {
        if should_assert {
            self.assert_not_referencing_factor_source(factor_instance.factor_source_id);
        }
        self.skipped.borrow_mut().insert(factor_instance)
    }

    pub(crate) fn add_signature_if_matching_factor(&self, signature: &HDSignature) {
        self.signed.borrow_mut().insert(signature)
    }

    fn new() -> Self {
        Self {
            signed: RefCell::new(PetitionWithFactorsStateFactors::<_>::new()),
            skipped: RefCell::new(PetitionWithFactorsStateFactors::<_>::new()),
        }
    }

    fn snapshot(&self) -> PetitionWithFactorsStateSnapshot {
        PetitionWithFactorsStateSnapshot {
            signed: self.signed.borrow().snapshot(),
            skipped: self.skipped.borrow().snapshot(),
        }
    }

    fn references_factor_source_by_id(&self, factor_source_id: FactorSourceID) -> bool {
        self.signed
            .borrow()
            .references_factor_source_by_id(factor_source_id)
            || self
                .skipped
                .borrow()
                .references_factor_source_by_id(factor_source_id)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct PetitionWithFactorsInput {
    /// Factors to sign with.
    factors: IndexSet<FactorInstance>,

    /// Number of required factors to sign with.
    required: i8,
}

impl PetitionWithFactorsInput {
    fn new(factors: IndexSet<FactorInstance>, required: i8) -> Self {
        Self { factors, required }
    }
    fn new_threshold(factors: IndexSet<FactorInstance>, threshold: i8) -> Self {
        Self::new(factors, threshold)
    }
    fn new_override(factors: IndexSet<FactorInstance>) -> Self {
        Self::new(factors, 1) // we need just one, anyone, factor for threshold.
    }
}

impl PetitionWithFactorsInput {
    pub fn reference_factor_source(&self, factor_source: &FactorSource) -> Option<&FactorInstance> {
        self.factors
            .iter()
            .find(|f| f.factor_source_id == factor_source.id)
    }
    pub fn references_factor_source(&self, factor_source: &FactorSource) -> bool {
        self.reference_factor_source(factor_source).is_some()
    }

    fn factors_count(&self) -> i8 {
        self.factors.len() as i8
    }

    fn remaining_factors_until_success(&self, snapshot: PetitionWithFactorsStateSnapshot) -> i8 {
        self.required - snapshot.signed_count()
    }

    fn is_fulfilled_by(&self, snapshot: PetitionWithFactorsStateSnapshot) -> bool {
        self.remaining_factors_until_success(snapshot) <= 0
    }

    fn factors_left_to_prompt(&self, snapshot: PetitionWithFactorsStateSnapshot) -> i8 {
        self.factors_count() - snapshot.prompted_count()
    }

    fn is_failure_with(&self, snapshot: PetitionWithFactorsStateSnapshot) -> bool {
        self.factors_left_to_prompt(snapshot) < self.required
    }
}

pub enum PetitionForFactorListStatus {
    /// In progress, still gathering signatures
    InProgress,

    Finished(PetitionForFactorListStatusFinished),
}

pub enum PetitionForFactorListStatusFinished {
    Success,
    Fail,
}

impl PetitionWithFactors {
    fn state_snapshot(&self) -> PetitionWithFactorsStateSnapshot {
        self.state.borrow().snapshot()
    }

    fn is_finished_successfully(&self) -> bool {
        self.input.is_fulfilled_by(self.state_snapshot())
    }

    fn is_finished_with_fail(&self) -> bool {
        self.input.is_failure_with(self.state_snapshot())
    }

    fn finished_with(&self) -> Option<PetitionForFactorListStatusFinished> {
        if self.is_finished_successfully() {
            Some(PetitionForFactorListStatusFinished::Success)
        } else if self.is_finished_with_fail() {
            Some(PetitionForFactorListStatusFinished::Fail)
        } else {
            None
        }
    }
}

impl PetitionWithFactors {
    pub fn status(&self) -> PetitionForFactorListStatus {
        if let Some(finished_state) = self.finished_with() {
            return PetitionForFactorListStatus::Finished(finished_state);
        }
        PetitionForFactorListStatus::InProgress
    }
}

/// =========================================
use std::{cell::RefCell, collections::BTreeSet};

use crate::prelude::*;

pub(crate) struct Petitions {
    /// Lookup from factor to TXID.
    ///
    ///
    /// The same FactorSource might be required by many payloads
    /// and per payload might be required by many entities, e.g. transactions
    /// `t0` and `t1`, where
    /// `t0` is signed by accounts: A and B
    /// `t1` is signed by accounts: A, C and D,
    ///
    /// Where A, B, C and D, all use the factor source, e.g. some arculus
    /// card which the user has setup as a factor (source) for all these accounts.
    pub factor_to_txid: HashMap<FactorSourceID, IndexSet<IntentHash>>,

    /// Lookup from TXID to signatures builders, sorted according to the order of
    /// transactions passed to the SignaturesBuilder.
    pub txid_to_petition: RefCell<IndexMap<IntentHash, PetitionOfTransaction>>,
}
impl Petitions {
    pub(crate) fn new(
        factor_to_txid: HashMap<FactorSourceID, IndexSet<IntentHash>>,
        txid_to_petition: IndexMap<IntentHash, PetitionOfTransaction>,
    ) -> Self {
        Self {
            factor_to_txid,
            txid_to_petition: RefCell::new(txid_to_petition),
        }
    }

    pub(super) fn continue_if_necessary(&self) -> Result<()> {
        self.txid_to_petition
            .borrow()
            .iter()
            .try_for_each(|(_, petition)| {
                petition
                    .for_entities
                    .borrow()
                    .iter()
                    .try_for_each(|(_, petition)| petition.continue_if_necessary())
            })
    }

    pub(crate) fn input_for_parallel_batch_driver(
        &self,
        factor_source: FactorSource,
    ) -> BatchTXBatchKeySigningRequest {
        let factor_source_id = factor_source.id.clone();
        let txids = self.factor_to_txid.get(&factor_source_id).unwrap();
        let per_transaction = txids
            .into_iter()
            .map(|txid| {
                let binding = self.txid_to_petition.borrow();
                let value = binding.get(txid).unwrap();
                value.input_for_parallel_batch_driver(factor_source.clone())
            })
            .collect::<IndexSet<BatchKeySigningRequest>>();

        BatchTXBatchKeySigningRequest::new(factor_source_id, per_transaction)
    }

    pub(crate) fn process_batch_response(
        &self,
        response: BatchSigningResponse,
        factor_sources: IndexSet<FactorSource>,
    ) {
        //    for factor_source in factor_sources {
        //     self.factor_to_txid.
        //    }
        for (factor_source_id, signatures) in response.signatures {
            for signature in signatures {
                let binding = self.txid_to_petition.borrow();
                let petition = binding.get(&signature.intent_hash).unwrap();
                petition.add_signature(signature)
            }
        }
    }

    // pub(super) fn process_outcome(
    //     &self,
    //     outcome: SignWithFactorSourceOrSourcesOutcome,
    //     factor_sources: IndexSet<FactorSource>,
    // ) {
    //     for factor_source in factor_sources {
    //         let txids = self.factor_to_txid.get(&factor_source.id).unwrap();
    //         for txid in txids {
    //             let binding = self.txid_to_petition.borrow();
    //             let petition = binding.get(txid).unwrap();
    //             petition.process_outcome(&outcome, &factor_source);
    //         }
    //     }
    // }
}

/// Essentially a wrapper around `IndexSet<PetitionOfTransactionByEntity>>`.
pub(crate) struct PetitionOfTransaction {
    /// Hash of transaction to sign
    pub intent_hash: IntentHash,

    pub for_entities:
        RefCell<HashMap<AccountAddressOrIdentityAddress, PetitionOfTransactionByEntity>>,
}

impl PetitionOfTransaction {
    pub fn all_factor_instances(&self) -> IndexSet<OwnedFactorInstance> {
        self.for_entities
            .borrow()
            .iter()
            .flat_map(|(_, petition)| petition.all_factor_instances())
            .collect()
    }

    pub fn add_signature(&self, signature: HDSignature) {
        let mut for_entities = self.for_entities.borrow_mut();
        let mut for_entity = for_entities
            .get(&signature.owned_factor_instance.owner)
            .unwrap();
        for_entity.add_signature(signature)
    }

    pub(crate) fn input_for_parallel_batch_driver(
        &self,
        factor_source: FactorSource,
    ) -> BatchKeySigningRequest {
        BatchKeySigningRequest::new(
            self.intent_hash.clone(),
            factor_source.id,
            self.all_factor_instances(),
        )
    }

    pub fn invalid_transactions_if_skipped(
        &self,
        factor_source: &FactorSource,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        self.for_entities
            .borrow()
            .iter()
            .flat_map(|(_, petition)| petition.invalid_transactions_if_skipped(factor_source))
            .collect()
    }

    pub(crate) fn process_batch_response(
        &self,
        response: BatchSigningResponse,
        factor_sources: IndexSet<FactorSource>,
    ) {
        todo!()
    }

    // pub fn process_outcome(
    //     &self,
    //     outcome: &SignWithFactorSourceOrSourcesOutcome,
    //     factor_source: &FactorSource,
    // ) {
    //     self.for_entities.borrow_mut().iter().for_each(|petition| {
    //         petition.process_outcome(outcome, factor_source);
    //     });
    // }
}

impl PetitionOfTransaction {
    pub(crate) fn new(
        intent_hash: IntentHash,
        for_entities: HashMap<AccountAddressOrIdentityAddress, PetitionOfTransactionByEntity>,
    ) -> Self {
        Self {
            intent_hash,
            for_entities: RefCell::new(for_entities),
        }
    }
}
