use crate::prelude::*;

pub struct BatchUseFactorSourceResponse<ID, Product>
where
    ID: Hash,
    Product: HasHDPublicKey,
{
    outputs: HashMap<ID, Vec<Product>>,
}

pub trait UseFactorSourceResponse<ID, Product>: UseFactorSourceRequestResponse
where
    ID: Hash + Clone,
    Product: HasHDPublicKey,
{
    fn products(&self) -> HashMap<ID, Vec<Product>>;
}

impl<ID, Product> UseFactorSourceRequestResponse for BatchUseFactorSourceResponse<ID, Product>
where
    ID: Hash,
    Product: HasHDPublicKey,
{
    fn factor_source_id(&self) -> HashSet<FactorSourceID> {
        HashSet::from_iter(
            self.outputs
                .values()
                .clone()
                .into_iter()
                .flat_map(|x| x.into_iter().map(|y| y.factor_source_id())),
        )
    }
}

impl<ID, Product> UseFactorSourceResponse<ID, Product> for BatchUseFactorSourceResponse<ID, Product>
where
    ID: Hash + Clone,
    Product: HasHDPublicKey,
{
    fn products(&self) -> HashMap<ID, Vec<Product>> {
        self.outputs.clone()
    }
}
