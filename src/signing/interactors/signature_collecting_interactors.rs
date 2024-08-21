use crate::prelude::*;

/// A collection of "interactors" which can sign transactions.
pub trait SignatureCollectingInteractors {
    fn interactor_for(&self, kind: FactorSourceKind) -> SigningInteractor;
}
