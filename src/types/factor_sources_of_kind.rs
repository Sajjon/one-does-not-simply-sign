use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FactorSourcesOfKind {
    pub(crate) kind: FactorSourceKind,
    factor_sources: Vec<HDFactorSource>,
}

impl FactorSourcesOfKind {
    pub(crate) fn new(
        kind: FactorSourceKind,
        factor_sources: impl IntoIterator<Item = HDFactorSource>,
    ) -> Result<Self> {
        let factor_sources = factor_sources.into_iter().collect::<IndexSet<_>>();
        if factor_sources.is_empty() {
            return Err(CommonError::FactorSourcesOfKindEmptyFactors);
        }
        if factor_sources
            .iter()
            .any(|f| f.factor_source_kind() != kind)
        {
            return Err(CommonError::InvalidFactorSourceKind);
        }
        Ok(Self {
            kind,
            factor_sources: factor_sources.into_iter().collect(),
        })
    }

    pub(crate) fn factor_sources(&self) -> IndexSet<HDFactorSource> {
        self.factor_sources.clone().into_iter().collect()
    }

    pub(crate) fn factor_source_ids(&self) -> Vec<FactorSourceIDFromHash> {
        self.factor_sources
            .iter()
            .map(|f| f.factor_source_id())
            .collect()
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
            Sut::new(FactorSourceKind::Device, [HDFactorSource::arculus()]),
            Err(CommonError::InvalidFactorSourceKind)
        );
    }

    #[test]
    fn invalid_two_two() {
        assert_eq!(
            Sut::new(
                FactorSourceKind::Device,
                [
                    HDFactorSource::arculus(),
                    HDFactorSource::device(),
                    HDFactorSource::arculus(),
                    HDFactorSource::device()
                ]
            ),
            Err(CommonError::InvalidFactorSourceKind)
        );
    }

    #[test]
    fn valid_one() {
        let sources = IndexSet::<HDFactorSource>::from_iter([HDFactorSource::device()]);
        let sut = Sut::new(FactorSourceKind::Device, sources.clone()).unwrap();
        assert_eq!(sut.factor_sources(), sources);
    }

    #[test]
    fn valid_two() {
        let sources = IndexSet::<HDFactorSource>::from_iter([
            HDFactorSource::ledger(),
            HDFactorSource::ledger(),
        ]);
        let sut = Sut::new(FactorSourceKind::Ledger, sources.clone()).unwrap();
        assert_eq!(sut.factor_sources(), sources);
    }
}
