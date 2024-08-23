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
}

impl HasSampleValues for PetitionFactorsStateSnapshot {
    fn sample() -> Self {
        Self::new(
            IndexSet::from_iter([HDSignature::sample(), HDSignature::sample_other()]),
            IndexSet::from_iter([
                HierarchicalDeterministicFactorInstance::sample(),
                HierarchicalDeterministicFactorInstance::sample_other(),
            ]),
        )
    }
    fn sample_other() -> Self {
        Self::new(
            IndexSet::from_iter([HDSignature::sample_other()]),
            IndexSet::from_iter([HierarchicalDeterministicFactorInstance::sample_other()]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Sut = PetitionFactorsStateSnapshot;

    #[test]
    fn equality() {
        assert_eq!(Sut::sample(), Sut::sample());
        assert_eq!(Sut::sample_other(), Sut::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(Sut::sample(), Sut::sample_other())
    }

    #[test]
    fn debug() {
        assert_eq!(format!("{:?}", Sut::sample()), "signatures: \"HDSignature { input: HDSignatureInput { intent_hash: TXID(\\\"dedede\\\"), owned_factor_instance: acco_Alice: factor_source_id: Device:dededede-dede-dede-dede-dededededede, derivation_path: 0/A/tx/0 } }, HDSignature { input: HDSignatureInput { intent_hash: TXID(\\\"ababab\\\"), owned_factor_instance: ident_Alice: factor_source_id: Ledger:1e1e1e1e-1e1e-1e1e-1e1e-1e1e1e1e1e1e, derivation_path: 0/A/tx/1 } }\", skipped: \"factor_source_id: Device:dededede-dede-dede-dede-dededededede, derivation_path: 0/A/tx/0, factor_source_id: Ledger:1e1e1e1e-1e1e-1e1e-1e1e-1e1e1e1e1e1e, derivation_path: 0/A/tx/1\"");
    }
}
