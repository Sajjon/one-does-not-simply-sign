use std::{
    borrow::Borrow,
    cell::{Ref, RefCell, RefMut},
};

use crate::prelude::*;

/// === FIA ===

struct FiaDependencies<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    drivers: Vec<Box<dyn UseFactorSourceDriver<ID, Path, Product>>>,
}
impl<ID, Path, Product> FiaDependencies<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    fn new(drivers: Vec<Box<dyn UseFactorSourceDriver<ID, Path, Product>>>) -> Self {
        Self { drivers }
    }

    fn driver_for_factor_source_of_kind(
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

pub struct FiaOutput<ID, Product>
where
    ID: Hash,
    Product: HasHDPublicKey,
{
    pub skipped_factor_sources: Vec<FactorSource>,
    pub outputs: HashMap<ID, Vec<Product>>,
}

struct FiaState<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    phantom_id: PhantomData<ID>,
    phantom_path: PhantomData<Path>,
    phantom_product: PhantomData<Product>,
    /// If this FIA is used to derive public keys we cannot skip any
    /// factor source, we need to derive ALL keys, so this variable will
    /// be `false`. However, for transaction signing, this variable will
    /// be `true`, since we **might** be able to skip *some* factor sources
    /// and have valid signed transactions with e.g. just override factors.
    supports_skipping_of_factor_sources: bool,

    /// Factor Sources Left to use. When this is Vec empty, we are done.
    factor_sources: Vec<FactorSource>,
}
impl<ID, Path, Product> FiaState<ID, Path, Product>
where
    ID: Hash + 'static,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    fn new(supports_skipping_of_factor_sources: bool, factor_sources: Vec<FactorSource>) -> Self {
        Self {
            supports_skipping_of_factor_sources,
            phantom_id: PhantomData,
            phantom_path: PhantomData,
            phantom_product: PhantomData,
            factor_sources,
        }
    }

    type DriverRequest = BatchUseFactorSourceRequest<ID, Path>;
    type DriverResponse = BatchUseFactorSourceResponse<ID, Product>;

    fn next_factor_sources(&self) -> Option<FactorSourcesOfKind> {
        todo!()
    }

    fn request_for(&self, factor_sources_of_kind: &FactorSourcesOfKind) -> Self::DriverRequest {
        let supports_parallelism = factor_sources_of_kind.kind.supports_parallelism();
        let supports_skipping = self.supports_skipping_of_factor_sources;
        let inputs = if supports_parallelism {
            HashMap::new()
        } else {
            HashMap::new()
        };
        if supports_skipping {
            Self::DriverRequest::new_skippable(|_| Vec::new(), HashMap::new())
        } else {
            Self::DriverRequest::new_unskippable(inputs)
        }
    }

    fn handle_response(&mut self, response: Self::DriverResponse) -> Result<()> {
        todo!()
    }

    fn accumulated_output(&self) -> Result<FiaOutput<ID, Product>> {
        todo!()
    }
}

pub struct FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    state: RefCell<FiaState<ID, Path, Product>>,
    dependencies: FiaDependencies<ID, Path, Product>,
}

/// ===== Public =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash + 'static,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
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

struct FactorSourcesOfKind {
    kind: FactorSourceKind,
    factor_sources: Vec<FactorSource>,
}
impl FactorSourcesOfKind {
    fn new(kind: FactorSourceKind, factor_sources: Vec<FactorSource>) -> Result<Self> {
        if factor_sources.iter().all(|f| f.kind == kind) {
            return Err(Error::InvalidFactorSourceKind);
        }
        Ok(Self {
            kind,
            factor_sources,
        })
    }
}

/// ===== Private Non Static =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash + 'static,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
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
        let request = self.state().request_for(factor_sources_of_kind);
        let response = driver.use_factor(request).await?;
        self.mut_state().handle_response(response)
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
    ID: Hash + 'static,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
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
