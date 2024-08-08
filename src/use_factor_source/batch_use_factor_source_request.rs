use crate::prelude::*;

pub struct BatchUseFactorSourceRequest<ID, Path>
where
    ID: Hash,
    Path: HasDerivationPath,
{
    inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>,
}

impl<ID, Path> BatchUseFactorSourceRequest<ID, Path>
where
    ID: Hash,
    Path: HasDerivationPath,
{
    pub fn new(inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>) -> Self {
        Self { inputs }
    }
}
