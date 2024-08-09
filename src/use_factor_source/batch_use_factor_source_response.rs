use crate::prelude::*;

pub struct BatchUseFactorSourceResponse<ID, Product>
where
    ID: Hash,
    Product: HasHDPublicKey,
{
    outputs: HashMap<ID, Vec<Product>>,
}
