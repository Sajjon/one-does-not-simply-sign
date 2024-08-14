#![allow(clippy::non_canonical_partial_ord_impl)]

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
    pub fn outcome(self) -> SignaturesOutcome {
        let txid_to_petition = self.txid_to_petition.into_inner();
        let mut failed_transactions = MaybeSignedTransactions::empty();
        let mut successful_transactions = MaybeSignedTransactions::empty();
        let mut skipped_factor_sources = IndexSet::<_>::new();
        for (txid, petition_of_transaction) in txid_to_petition.into_iter() {
            let (successful, signatures, skipped) = petition_of_transaction.outcome();
            if successful {
                successful_transactions.add_signatures(txid, signatures);
            } else {
                failed_transactions.add_signatures(txid, signatures);
            }
            skipped_factor_sources.extend(skipped)
        }

        SignaturesOutcome::new(
            successful_transactions,
            failed_transactions,
            skipped_factor_sources,
        )
    }

    pub(crate) fn new(
        factor_to_txid: HashMap<FactorSourceID, IndexSet<IntentHash>>,
        txid_to_petition: IndexMap<IntentHash, PetitionOfTransaction>,
    ) -> Self {
        Self {
            factor_to_txid,
            txid_to_petition: RefCell::new(txid_to_petition),
        }
    }

    /// `Ok(true)` means "continue", `Ok(false)` means "stop, we are done". `Err(_)` means "stop, we have failed".
    pub fn continue_if_necessary(&self) -> Result<bool> {
        let should_continue_signals = self
            .txid_to_petition
            .borrow()
            .iter()
            .flat_map(|(_, petition)| {
                petition
                    .for_entities
                    .borrow()
                    .iter()
                    .map(|(_, petition)| petition.continue_if_necessary())
                    .collect_vec()
            })
            .collect::<Result<Vec<bool>>>()?;

        let should_continue_signal = should_continue_signals.into_iter().all(|b| b);
        Ok(should_continue_signal)
    }

    pub fn invalid_transactions_if_skipped(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        let txids = self.factor_to_txid.get(factor_source_id).unwrap();
        txids
            .into_iter()
            .flat_map(|txid| {
                let binding = self.txid_to_petition.borrow();
                let value = binding.get(txid).unwrap();
                value.invalid_transactions_if_skipped(factor_source_id)
            })
            .collect::<IndexSet<_>>()
    }

    pub(crate) fn input_for_parallel_batch_interactor(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> BatchTXBatchKeySigningRequest {
        let txids = self.factor_to_txid.get(factor_source_id).unwrap();
        let per_transaction = txids
            .into_iter()
            .map(|txid| {
                let binding = self.txid_to_petition.borrow();
                let petition = binding.get(txid).unwrap();
                petition.input_for_parallel_batch_interactor(factor_source_id)
            })
            .collect::<IndexSet<BatchKeySigningRequest>>();

        BatchTXBatchKeySigningRequest::new(*factor_source_id, per_transaction)
    }

    fn add_signature(&self, signature: &HDSignature) {
        let binding = self.txid_to_petition.borrow();
        let petition = binding.get(signature.intent_hash()).unwrap();
        petition.add_signature(signature.clone())
    }

    fn skip_factor_source_with_id(&self, skipped_factor_source_id: &FactorSourceID) {
        let binding = self.txid_to_petition.borrow();
        let txids = self.factor_to_txid.get(skipped_factor_source_id).unwrap();
        txids.into_iter().for_each(|txid| {
            let petition = binding.get(txid).unwrap();
            petition.skipped_factor_source(skipped_factor_source_id)
        });
    }

    pub(crate) fn process_batch_response(
        &self,
        response: SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>,
    ) {
        match response {
            SignWithFactorSourceOrSourcesOutcome::Signed {
                produced_signatures,
            } => {
                produced_signatures
                    .signatures
                    .values()
                    .flatten()
                    .for_each(|s| self.add_signature(s));
            }
            SignWithFactorSourceOrSourcesOutcome::Skipped {
                ids_of_skipped_factors_sources,
            } => {
                for skipped_factor_source_id in ids_of_skipped_factors_sources.iter() {
                    self.skip_factor_source_with_id(skipped_factor_source_id)
                }
            }
        }
    }
}

/// Essentially a wrapper around `Iterator<Item = BuilderEntity>`.
pub(crate) struct PetitionOfTransaction {
    /// Hash of transaction to sign
    pub intent_hash: IntentHash,

    pub for_entities: RefCell<HashMap<AccountAddressOrIdentityAddress, BuilderEntity>>,
}

impl PetitionOfTransaction {
    /// Returns `(true, _)` if this transaction has been successfully signed by
    /// all required factor instances.
    ///
    /// Returns `(false, _)` if not enough factor instances have signed.
    ///
    /// The second value in the tuple `(_, IndexSet<HDSignature>, _)` contains all
    /// the signatures, even if it the transaction was failed, all signatures
    /// will be returned (which might be empty).
    ///
    /// The third value in the tuple `(_, _, IndexSet<FactorSourceID>)` contains the
    /// id of all the factor sources which was skipped.
    pub fn outcome(self) -> (bool, IndexSet<HDSignature>, IndexSet<FactorSourceID>) {
        let for_entities = self
            .for_entities
            .into_inner()
            .values()
            .map(|x| x.to_owned())
            .collect_vec();

        let successful = for_entities
            .iter()
            .all(|b| b.has_signatures_requirement_been_fulfilled());

        let signatures = for_entities
            .iter()
            .flat_map(|x| x.all_signatures())
            .collect::<IndexSet<_>>();

        let skipped = for_entities
            .iter()
            .flat_map(|x| x.all_skipped_factor_sources())
            .collect::<IndexSet<_>>();

        (successful, signatures, skipped)
    }

    fn _all_factor_instances(&self) -> IndexSet<OwnedFactorInstance> {
        self.for_entities
            .borrow()
            .iter()
            .flat_map(|(_, petition)| petition.all_factor_instances())
            .collect()
    }

    pub fn all_factor_instances_of_source(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> IndexSet<OwnedFactorInstance> {
        self._all_factor_instances()
            .into_iter()
            .filter(|f| f.factor_instance().factor_source_id == *factor_source_id)
            .collect()
    }

    pub fn add_signature(&self, signature: HDSignature) {
        let for_entities = self.for_entities.borrow_mut();
        let for_entity = for_entities
            .get(&signature.owned_factor_instance().owner)
            .unwrap();
        for_entity.add_signature(signature.clone());
    }

    pub fn skipped_factor_source(&self, factor_source_id: &FactorSourceID) {
        let mut for_entities = self.for_entities.borrow_mut();
        for petition in for_entities.values_mut() {
            petition.skipped_factor_source_if_relevant(factor_source_id)
        }
    }

    pub(crate) fn input_for_parallel_batch_interactor(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> BatchKeySigningRequest {
        BatchKeySigningRequest::new(
            self.intent_hash.clone(),
            *factor_source_id,
            self.all_factor_instances_of_source(factor_source_id),
        )
    }

    pub fn invalid_transactions_if_skipped(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> IndexSet<InvalidTransactionIfSkipped> {
        self.for_entities
            .borrow()
            .iter()
            .flat_map(|(_, petition)| petition.invalid_transactions_if_skipped(factor_source_id))
            .collect()
    }

    pub(crate) fn new(
        intent_hash: IntentHash,
        for_entities: HashMap<AccountAddressOrIdentityAddress, BuilderEntity>,
    ) -> Self {
        Self {
            intent_hash,
            for_entities: RefCell::new(for_entities),
        }
    }
}
