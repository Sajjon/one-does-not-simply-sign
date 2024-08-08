use std::borrow::Borrow;

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
    type Driver = dyn UseFactorSourceDriver<ID, Path, Product>;
    type BoxedDriver = Box<Self::Driver>;
    type DriverRequest = BatchUseFactorSourceRequest<ID, Path>;
    type DriverResponse = BatchUseFactorSourceResponse<ID, Product>;

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
            self.reduce(factor_source).await?;

            if self.is_done() {
                break;
            }
        }
        self.accumulated_response()
    }
}

/// ===== Private Non Static =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    async fn reduce(&self, factor_source: &FactorSource) -> Result<()> {
        let driver = self.driver_for_factor_source(factor_source);
        let request = self.request_for(factor_source);
        let response = driver.use_factor(request).await?;
        self.handle_response(response)
    }

    fn request_for(&self, factor_source: &FactorSource) -> Self::DriverRequest {
        todo!()
    }

    fn handle_response(&self, response: Self::DriverResponse) -> Result<()> {
        todo!()
    }

    fn is_done(&self) -> bool {
        todo!()
    }

    fn accumulated_response(&self) -> Result<BatchUseFactorSourceResponse<ID, Product>> {
        todo!()
    }
}

/// ===== Private Static =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    fn driver_for_factor_source(
        &self,
        factor_source: &FactorSource,
    ) -> &dyn UseFactorSourceDriver<ID, Path, Product> {
        self.drivers
            .iter()
            .find(|d| d.can_be_used_for(factor_source))
            .unwrap()
            .borrow()
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
    ) -> Result<Vec<Self::BoxedDriver>> {
        todo!()
    }
}
