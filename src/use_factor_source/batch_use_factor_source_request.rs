use crate::prelude::*;

pub struct SkipDelegate<ID, I, S>
where
    I: Fn(FactorSourceID) -> Vec<ID>,
    S: Fn(FactorSourceID) -> (),
{
    fn_invalid_if_skipped: I,
    fn_skip: S,
}

impl<ID, I, S> SkipDelegate<ID, I, S>
where
    I: Fn(FactorSourceID) -> Vec<ID>,
    S: Fn(FactorSourceID) -> (),
{
    pub fn new(invalid_if_skipped: I, skip: S) -> Self {
        Self {
            fn_invalid_if_skipped: invalid_if_skipped,
            fn_skip: skip,
        }
    }

    pub fn invalid_if_skipped(&self, factor_source_id: FactorSourceID) -> Vec<ID> {
        (self.fn_invalid_if_skipped)(factor_source_id)
    }

    pub fn skip(&self, factor_source_id: FactorSourceID) {
        (self.fn_skip)(factor_source_id)
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
    skipping: Option<SkipDelegate>,

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
    pub fn new_skippable<I, S>(
        invalid_if_skipped: I,
        skip: S,
        inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>,
    ) -> Self
    where
        I: Fn(FactorSourceID) -> Vec<ID>,
        S: Fn(FactorSourceID) -> (),
    {
        Self::new(
            Some(Box::new(SkipDelegateImpl::new(invalid_if_skipped, skip))),
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
        skipping: Option<Box<dyn SkipDelegate<ID> + Send>>,
        inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>,
    ) -> Self {
        Self { skipping, inputs }
    }

    pub fn new_unskippable(inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>) -> Self {
        Self::new(None, inputs)
    }
}
