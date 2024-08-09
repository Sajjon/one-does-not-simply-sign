use crate::prelude::*;

pub(super) struct FiaDependencies<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    pub(super) drivers: Vec<Box<dyn UseFactorSourceDriver<ID, Path, Product>>>,
}

impl<ID, Path, Product> FiaDependencies<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    pub(super) fn new(drivers: Vec<Box<dyn UseFactorSourceDriver<ID, Path, Product>>>) -> Self {
        Self { drivers }
    }

    pub(super) fn driver_for_factor_source_of_kind(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> &dyn UseFactorSourceDriver<ID, Path, Product> {
        self.drivers
            .iter()
            .find(|d| d.supports(factor_source_kind))
            .unwrap()
            .borrow()
    }
}
