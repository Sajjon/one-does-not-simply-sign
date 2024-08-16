use crate::prelude::*;

pub type SignaturesCollectorDependencies = CollectorDependencies<SignatureCollectingInteractors>;

impl SignaturesCollectorDependencies {
    pub fn new(
        interactors: Arc<dyn SignatureCollectingInteractors>,
        factors_of_kind: IndexSet<FactorSourcesOfKind>,
    ) -> Self {
        Self {
            interactors,
            factors_of_kind,
        }
    }
}
