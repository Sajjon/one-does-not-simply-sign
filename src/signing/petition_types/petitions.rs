#![allow(clippy::non_canonical_partial_ord_impl)]

use crate::prelude::*;

#[derive(derive_more::Debug)]
#[debug("{}", self.debug_str())]
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
    pub txid_to_petition: RefCell<IndexMap<IntentHash, PetitionTransaction>>,
}

impl Petitions {
    fn debug_str(&self) -> String {
        self.txid_to_petition
            .borrow()
            .iter()
            .map(|p| format!("{:?}: {:?}", p.0, p.1))
            .join("\n")
    }
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
        txid_to_petition: IndexMap<IntentHash, PetitionTransaction>,
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

    pub(crate) fn input_for_interactor(
        &self,
        factor_source_id: &FactorSourceID,
    ) -> BatchTXBatchKeySigningRequest {
        let txids = self.factor_to_txid.get(factor_source_id).unwrap();
        let per_transaction = txids
            .into_iter()
            .map(|txid| {
                let binding = self.txid_to_petition.borrow();
                let petition = binding.get(txid).unwrap();
                petition.input_for_interactor(factor_source_id)
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
