use std::process::Output;

use crate::prelude::*;

pub struct FiaOutputReducer<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    id: ID,
    products: RefCell<HashMap<Path, Product>>,
}

impl<ID, Path, Product> FiaOutputReducer<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    fn reduce(&self, outputs: HashMap<ID, Vec<Product>>) -> Result<()> {
        // let mut m = self.products.borrow_mut();
        // for (id, product) in outputs.into_iter() {
        //     let path = product.der
        //     m.insert()
        // }
        // Ok(())
        todo!()
    }
}

/// Produce many signatures per transaction intent per FactorSource
pub type FiaTransactionSigning = FactorInstanceAccumulator<IntentHash, HDPublicKey, HDSignature>;

impl FiaTransactionSigning {
    fn new_batch_sign_transactions(
        inputs: HashMap<FactorSourceID, HashMap<IntentHash, Vec<HDPublicKey>>>,
        factor_sources: IndexSet<FactorSource>,
        all_drivers: impl IntoIterator<Item = Box<dyn SignWithFactorSourceDriver>>,
    ) -> Result<Self> {
        //        Self::new(BatchUseFactorSourceRequest::new(inputs), factor_sources)
        todo!()
    }

    fn new_batch_sign_transactions_grouping(
        instances_per_transaction: HashMap<IntentHash, Vec<HDPublicKey>>,
        factor_sources: IndexSet<FactorSource>,
        all_drivers: impl IntoIterator<Item = Box<dyn SignWithFactorSourceDriver>>,
    ) -> Result<Self> {
        // let inputs = ...
        // Self::new_batch_sign_transactions(inputs, factor_sources)
        todo!()
    }

    fn new_batch_sign_by_analyzing_transactions_using<F>(
        transactions: Vec<TransactionIntent>,
        entities: Vec<Entity>,
        factor_sources: IndexSet<FactorSource>,
        all_drivers: impl IntoIterator<Item = Box<dyn SignWithFactorSourceDriver>>,
        signers_of_transaction: F,
    ) -> Result<Self>
    where
        F: Fn(TransactionIntent) -> Vec<HDPublicKey>,
    {
        //    let inputs: HashMap<IntentHash, Vec<HDPublicKey>> = transaction.into_iter().
        //    Self::new_batch_sign_transactions_grouping(inputs, )
        todo!()
    }

    pub fn new_batch_sign_by_analyzing_transactions(
        transactions: Vec<TransactionIntent>,
        entities: Vec<Entity>,
        factor_sources: IndexSet<FactorSource>,
        all_drivers: impl IntoIterator<Item = Box<dyn SignWithFactorSourceDriver>>,
    ) -> Result<Self> {
        Self::new_batch_sign_by_analyzing_transactions_using(
            transactions,
            entities,
            factor_sources,
            all_drivers,
            |t| todo!(),
        )
    }
}
