use crate::prelude::*;

pub trait CanSkipDelegate<ID> {
    fn invalid_if_skipped(&self, factor_source_id: FactorSourceID) -> Vec<ID>;
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
}
