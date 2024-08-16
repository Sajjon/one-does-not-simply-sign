use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct FactorSourcesOfKind {
    pub(crate) kind: FactorSourceKind,
    factor_sources: Vec<FactorSource>,
}

impl FactorSourcesOfKind {
    pub(crate) fn new(
        kind: FactorSourceKind,
        factor_sources: impl IntoIterator<Item = FactorSource>,
    ) -> Result<Self> {
        let factor_sources = factor_sources.into_iter().collect::<IndexSet<_>>();
        if factor_sources.is_empty() {
            return Err(CommonError::FactorSourcesOfKindEmptyFactors);
        }
        if factor_sources.iter().any(|f| f.kind() != kind) {
            return Err(CommonError::InvalidFactorSourceKind);
        }
        Ok(Self {
            kind,
            factor_sources: factor_sources.into_iter().collect(),
        })
    }

    pub(crate) fn factor_sources(&self) -> IndexSet<FactorSource> {
        self.factor_sources.clone().into_iter().collect()
    }

    pub(crate) fn factor_source_ids(&self) -> Vec<FactorSourceID> {
        self.factor_sources.iter().map(|f| f.id).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type Sut = FactorSourcesOfKind;

    #[test]
    fn invalid_empty() {
        assert_eq!(
            Sut::new(FactorSourceKind::Device, []),
            Err(CommonError::FactorSourcesOfKindEmptyFactors)
        );
    }

    #[test]
    fn invalid_single_element() {
        assert_eq!(
            Sut::new(FactorSourceKind::Device, [FactorSource::arculus()]),
            Err(CommonError::InvalidFactorSourceKind)
        );
    }

    #[test]
    fn invalid_two_two() {
        assert_eq!(
            Sut::new(
                FactorSourceKind::Device,
                [
                    FactorSource::arculus(),
                    FactorSource::device(),
                    FactorSource::arculus(),
                    FactorSource::device()
                ]
            ),
            Err(CommonError::InvalidFactorSourceKind)
        );
    }

    #[test]
    fn valid_one() {
        let sources = IndexSet::<FactorSource>::from_iter([FactorSource::device()]);
        let sut = Sut::new(FactorSourceKind::Device, sources.clone()).unwrap();
        assert_eq!(sut.factor_sources(), sources);
    }

    #[test]
    fn valid_two() {
        let sources =
            IndexSet::<FactorSource>::from_iter([FactorSource::ledger(), FactorSource::ledger()]);
        let sut = Sut::new(FactorSourceKind::Ledger, sources.clone()).unwrap();
        assert_eq!(sut.factor_sources(), sources);
    }
}
