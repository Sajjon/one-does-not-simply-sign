use crate::prelude::*;

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
pub enum SignWithFactorSourceOrSourcesOutcome<T> {
    /// The user successfully signed with the factor source(s), the associated
    /// value contains the produces signatures and any relevant metadata.
    #[debug("Signed: {:#?}", produced_signatures)]
    Signed { produced_signatures: T },

    /// The user skipped signing with the factor sources with ids
    #[debug("Skipped")]
    Skipped {
        ids_of_skipped_factors_sources: Vec<FactorSourceID>,
    },
}

impl<T> SignWithFactorSourceOrSourcesOutcome<T> {
    pub fn signed(produced_signatures: T) -> Self {
        Self::Signed {
            produced_signatures,
        }
    }

    pub fn skipped(ids_of_skipped_factors_sources: IndexSet<FactorSourceID>) -> Self {
        Self::Skipped {
            ids_of_skipped_factors_sources: ids_of_skipped_factors_sources
                .into_iter()
                .collect_vec(),
        }
    }
    pub fn skipped_factor_source(factor_source_id: FactorSourceID) -> Self {
        Self::skipped(IndexSet::from_iter([factor_source_id]))
    }
}
