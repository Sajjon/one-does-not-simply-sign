use super::*;
use crate::prelude::*;

/// === FIA ===

pub struct FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash + Send + Sync,
    Path: HasDerivationPath + Send + Sync,
    Product: HasHDPublicKey + Send + Sync,
{
    state: RefCell<FiaState<ID, Path, Product>>,
    dependencies: FiaDependencies<ID, Path, Product>,
}

/// ===== Public =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash + Send + Sync + 'static,
    Path: HasDerivationPath + Send + Sync,
    Product: HasHDPublicKey + Send + Sync,
{
    type State = FiaState<ID, Path, Product>;
    type Dependencies = FiaDependencies<ID, Path, Product>;

    type Driver = dyn UseFactorSourceDriver<ID, Path, Product>;
    type BoxedDriver = Box<Self::Driver>;
    type DriverRequest = BatchUseFactorSourceRequest<ID, Path>;
    type DriverResponse = BatchUseFactorSourceResponse<ID, Product>;

    pub fn new(
        supports_skipping_of_factor_sources: bool,
        request: BatchUseFactorSourceRequest<ID, Path>,
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        all_drivers: impl IntoIterator<Item = Box<dyn UseFactorSourceDriver<ID, Path, Product>>>,
    ) -> Result<Self> {
        let factor_sources = Self::factor_sources_to_use(&request, all_factor_sources_in_profile)?;
        let drivers = Self::drivers_to_use(&factor_sources, all_drivers)?;

        let dependencies = Self::Dependencies::new(drivers);
        let state = FiaState::new(supports_skipping_of_factor_sources, factor_sources);

        Ok(Self {
            state: RefCell::new(state),
            dependencies,
        })
    }

    pub async fn accumulate(&self) -> Result<FiaOutput<ID, Product>> {
        while let Some(factor_sources) = self.next_factor_sources() {
            self.reduce(&factor_sources).await?;

            if self.is_done_early() {
                break;
            }
        }
        self.accumulated_output()
    }
}

/// ===== Private Non Static =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash + Send + Sync + 'static,
    Path: HasDerivationPath + Send + Sync,
    Product: HasHDPublicKey + Send + Sync,
{
    fn state(&self) -> Ref<FiaState<ID, Path, Product>> {
        self.state.borrow()
    }

    fn mut_state(&self) -> RefMut<FiaState<ID, Path, Product>> {
        self.state.borrow_mut()
    }

    fn next_factor_sources(&self) -> Option<FactorSourcesOfKind> {
        self.state().next_factor_sources()
    }

    async fn reduce(&self, factor_sources_of_kind: &FactorSourcesOfKind) -> Result<()> {
        let driver = self.driver_for_factor_source_of_kind(factor_sources_of_kind.kind);
        if factor_sources_of_kind.kind.supports_parallelism() {
            let parallel_request = self.state().parallel_request_for(factor_sources_of_kind);
            let outcome = driver.use_factors(parallel_request).await?;
            self.mut_state().handle_outcome(outcome)?
        } else {
            for factor_source in factor_sources_of_kind.factor_sources.iter() {
                let request = self.state().serial_request_for(factor_source);
                let outcome = driver.use_factors(request).await?;
                self.mut_state().handle_outcome(outcome)?
            }
        }
        Ok(())
    }

    fn driver_for_factor_source_of_kind(
        &self,
        factor_source_kind: FactorSourceKind,
    ) -> &dyn UseFactorSourceDriver<ID, Path, Product> {
        self.dependencies
            .driver_for_factor_source_of_kind(factor_source_kind)
    }

    fn is_done_early(&self) -> bool {
        if !self.state().supports_skipping_of_factor_sources {
            // Cannot skip any factor sources, so cannot be done "early".
            return false;
        }
        todo!()
    }

    fn accumulated_output(&self) -> Result<FiaOutput<ID, Product>> {
        self.state().accumulated_output()
    }
}

/// ===== Private Static =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash + Send + Sync + 'static,
    Path: HasDerivationPath + Send + Sync,
    Product: HasHDPublicKey + Send + Sync,
{
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
