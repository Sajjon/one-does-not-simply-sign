use crate::prelude::*;

/// === FIA ===

pub struct FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    phantom_id: PhantomData<ID>,
    phantom_path: PhantomData<Path>,
    phantom_product: PhantomData<Product>,

    factor_sources: Vec<FactorSource>,
    drivers: Vec<Box<dyn UseFactorSourceDriver<ID, Path, Product>>>,
}

/// ===== Public =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    pub fn new(
        request: BatchUseFactorSourceRequest<ID, Path>,
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        all_drivers: impl IntoIterator<Item = Box<dyn UseFactorSourceDriver<ID, Path, Product>>>,
    ) -> Result<Self> {
        let factor_sources = Self::factor_sources_to_use(&request, all_factor_sources_in_profile)?;

        let drivers = Self::drivers_to_use(&factor_sources, all_drivers)?;

        Ok(Self {
            phantom_id: PhantomData,
            phantom_path: PhantomData,
            phantom_product: PhantomData,
            factor_sources,
            drivers,
        })
    }

    pub async fn accumulate(&self) -> Result<BatchUseFactorSourceResponse<ID, Product>> {
        for factor_source in self.factor_sources.iter() {
            let driver = self.driver_for_factor_source(factor_source);
        }
        todo!()
    }
}

/// ===== Private =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    fn driver_for_factor_source(
        &self,
        factor_source: &FactorSource,
    ) -> &Box<dyn UseFactorSourceDriver<ID, Path, Product>> {
        self.drivers
            .iter()
            .find(|driver| driver.can_be_used_for(factor_source))
            .unwrap()
    }

    fn factor_sources_to_use(
        request: &BatchUseFactorSourceRequest<ID, Path>,
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
    ) -> Result<Vec<FactorSource>> {
        todo!()
    }

    fn drivers_to_use(
        factor_sources_to_use: &[FactorSource],
        all_drivers: impl IntoIterator<Item = Box<dyn UseFactorSourceDriver<ID, Path, Product>>>,
    ) -> Result<Vec<Box<dyn UseFactorSourceDriver<ID, Path, Product>>>> {
        todo!()
    }
}
