use crate::prelude::*;

pub(super) struct SignaturesCollectorState {
    pub(super) petitions: RefCell<Petitions>,
}
impl SignaturesCollectorState {
    fn with_petitions(petitions: Petitions) -> Self {
        Self {
            petitions: RefCell::new(petitions),
        }
    }

    pub(super) fn new(
        factor_to_txid: HashMap<FactorSourceID, IndexSet<IntentHash>>,
        txid_to_petition: IndexMap<IntentHash, PetitionOfTransaction>,
    ) -> Self {
        Self::with_petitions(Petitions::new(factor_to_txid, txid_to_petition))
    }
}
