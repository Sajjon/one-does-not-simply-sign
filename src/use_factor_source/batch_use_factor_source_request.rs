use crate::prelude::*;

pub struct BatchUseFactorSourceRequest<ID, Path>
where
    ID: Hash,
    Path: HasDerivationPath,
{
    /// Only signing supports skipping factor sources, key derivation does not,
    /// so this closure is `None` for key derivation.
    invalid_if_skipped: Option<Vec<ID>>,

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
        invalid_if_skipped: Option<Vec<ID>>,
        inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>,
    ) -> Self {
        Self {
            invalid_if_skipped,
            inputs,
        }
    }

    pub fn new_skippable(
        invalid_if_skipped: Option<Vec<ID>>,
        inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>,
    ) -> Self {
        Self::new(invalid_if_skipped, inputs)
    }

    pub fn new_unskippable(inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>) -> Self {
        Self::new(None, inputs)
    }
}
