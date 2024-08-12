use crate::prelude::*;

pub(super) struct FactorSourcesOfKind {
    pub(super) kind: FactorSourceKind,
    pub(super) factor_sources: Vec<FactorSource>,
}

impl FactorSourcesOfKind {
    pub(super) fn new(kind: FactorSourceKind, factor_sources: Vec<FactorSource>) -> Result<Self> {
        if factor_sources.iter().all(|f| f.kind == kind) {
            return Err(Error::InvalidFactorSourceKind);
        }
        Ok(Self {
            kind,
            factor_sources,
        })
    }
}
