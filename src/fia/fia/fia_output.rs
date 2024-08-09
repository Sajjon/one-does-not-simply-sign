use crate::prelude::*;

pub struct FiaOutput<ID, Product>
where
    ID: Hash,
    Product: HasHDPublicKey,
{
    pub skipped_factor_sources: Vec<FactorSource>,
    pub outputs: HashMap<ID, Vec<Product>>,
}
