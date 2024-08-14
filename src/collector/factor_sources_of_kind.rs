use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct FactorSourcesOfKind {
    pub(super) kind: FactorSourceKind,
    factor_sources: Vec<FactorSource>,
}

impl FactorSourcesOfKind {
    pub(super) fn new(
        kind: FactorSourceKind,
        factor_sources: impl IntoIterator<Item = FactorSource>,
    ) -> Result<Self> {
        let factor_sources = factor_sources.into_iter().collect::<IndexSet<_>>();
        if factor_sources.iter().any(|f| f.kind() != kind) {
            return Err(CommonError::InvalidFactorSourceKind);
        }
        Ok(Self {
            kind,
            factor_sources: factor_sources.into_iter().collect(),
        })
    }

    pub(super) fn factor_sources(&self) -> IndexSet<FactorSource> {
        self.factor_sources.clone().into_iter().collect()
    }

    pub(super) fn factor_source_ids(&self) -> Vec<FactorSourceID> {
        self.factor_sources.iter().map(|f| f.id).collect()
    }
}
