use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct BuilderFactorsStateSnapshot {
    /// Factors that have signed.
    signed: IndexSet<HDSignature>,

    /// Factors that user skipped.
    skipped: IndexSet<FactorInstance>,
}

impl BuilderFactorsStateSnapshot {
    pub(super) fn new(signed: IndexSet<HDSignature>, skipped: IndexSet<FactorInstance>) -> Self {
        Self { signed, skipped }
    }
    pub(super) fn prompted_count(&self) -> i8 {
        self.signed_count() + self.skipped_count()
    }

    pub(super) fn signed_count(&self) -> i8 {
        self.signed.len() as i8
    }

    fn skipped_count(&self) -> i8 {
        self.skipped.len() as i8
    }
}
