use crate::prelude::*;

/// An immutable "snapshot" of `PetitionFactorsState`
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub(super) struct PetitionFactorsStateSnapshot {
    /// Factors that have signed.
    signed: IndexSet<HDSignature>,

    /// Factors that user skipped.
    skipped: IndexSet<HierarchicalDeterministicFactorInstance>,
}

impl PetitionFactorsStateSnapshot {
    #[allow(unused)]
    fn debug_str(&self) -> String {
        let signatures = self
            .signed
            .clone()
            .into_iter()
            .map(|s| format!("{:#?}", s))
            .join(", ");

        let skipped = self
            .skipped
            .clone()
            .into_iter()
            .map(|s| format!("{:#?}", s))
            .join(", ");

        format!("signatures: {:#?}, skipped: {:#?}", signatures, skipped)
    }

    pub(super) fn new(
        signed: IndexSet<HDSignature>,
        skipped: IndexSet<HierarchicalDeterministicFactorInstance>,
    ) -> Self {
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
