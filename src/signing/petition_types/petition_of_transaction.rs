use crate::prelude::*;

/// Petition of signatures for a transaction.
/// Essentially a wrapper around `Iterator<Item = PetitionEntity>`.
#[derive(derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub(crate) struct PetitionTransaction {
    /// Hash of transaction to sign
    pub intent_hash: IntentHash,

    pub for_entities: RefCell<HashMap<AddressOfAccountOrPersona, PetitionEntity>>,
}

impl PetitionTransaction {
    fn debug_str(&self) -> String {
        let entities = self
            .for_entities
            .borrow()
            .iter()
            .map(|p| format!("PetitionEntity({:#?})", p.1))
            .join(", ");

        format!("PetitionTransaction(for_entities: [{}])", entities)
    }

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

    pub(crate) fn input_for_interactor(
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
        for_entities: HashMap<AddressOfAccountOrPersona, PetitionEntity>,
    ) -> Self {
        Self {
            intent_hash,
            for_entities: RefCell::new(for_entities),
        }
    }
}
