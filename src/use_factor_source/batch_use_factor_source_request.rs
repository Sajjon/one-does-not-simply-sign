use crate::prelude::*;

pub trait CanSkipDelegate<ID> {
    fn invalid_if_skipped(&self, factor_source_id: FactorSourceID) -> Vec<ID>;
}

pub struct CanSkipDelegateImpl<ID> {
    fn_invalid_if_skipped: Box<dyn Fn(FactorSourceID) -> Vec<ID> + Send + 'static>,
}
impl<ID> CanSkipDelegateImpl<ID> {
    pub fn new<F>(invalid_if_skipped: F) -> Self
    where
        F: Fn(FactorSourceID) -> Vec<ID> + Send + 'static,
    {
        Self {
            fn_invalid_if_skipped: Box::new(invalid_if_skipped),
        }
    }
}
impl<ID> CanSkipDelegate<ID> for CanSkipDelegateImpl<ID> {
    fn invalid_if_skipped(&self, factor_source_id: FactorSourceID) -> Vec<ID> {
        (self.fn_invalid_if_skipped)(factor_source_id)
    }
}

pub struct BatchUseFactorSourceRequest<ID, Path>
where
    ID: Hash,
    Path: HasDerivationPath,
{
    /// Only signing supports skipping factor sources, key derivation does not,
    /// so this closure is `None` for key derivation.
    ///
    /// We CANNOT represent this as a `HashMap<FactorSourceID, Vec<ID>>` because,
    /// if we skip with Factor `A`, that might affect the possibility of skipping
    /// with Factor `B`. Therefore some kind of closure / callback is needed.
    invalid_if_skipped: Option<Box<dyn CanSkipDelegate<ID> + Send>>,

    /// Inputs will only contain multiple `FactorSourceID` for when
    /// DeviceFactorSource is used, which is the only kind which supports
    /// "parallel" usage, i.e. we wanna prompt user for biometrics just
    /// once, and load all mnemonics and then sequentially use each
    /// DeviceFactorSource.
    inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>,
}

impl<ID, Path> BatchUseFactorSourceRequest<ID, Path>
where
    ID: Hash + 'static,
    Path: HasDerivationPath,
{
    pub fn new_skippable<F>(
        invalid_if_skipped: F,
        inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>,
    ) -> Self
    where
        F: Fn(FactorSourceID) -> Vec<ID> + Send + 'static,
    {
        Self::new(
            Some(Box::new(CanSkipDelegateImpl::new(invalid_if_skipped))),
            inputs,
        )
    }
}

impl<ID, Path> BatchUseFactorSourceRequest<ID, Path>
where
    ID: Hash,
    Path: HasDerivationPath,
{
    pub fn new(
        invalid_if_skipped: Option<Box<dyn CanSkipDelegate<ID> + Send>>,
        inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>,
    ) -> Self {
        Self {
            invalid_if_skipped,
            inputs,
        }
    }

    pub fn new_unskippable(inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>) -> Self {
        Self::new(None, inputs)
    }
}
