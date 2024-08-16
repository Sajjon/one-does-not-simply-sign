use crate::prelude::*;

/// A collection of "interactors" which can produce output from factor sources.
pub trait IsFactorSourceInteractorCollection<T> {
    fn interactor_for(&self, kind: FactorSourceKind) -> InteractorParallelOrSerial;
}
