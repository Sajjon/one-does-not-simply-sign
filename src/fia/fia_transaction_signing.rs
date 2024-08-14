use std::process::Output;

use crate::prelude::*;

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
        factor_sources: IndexSet<FactorSource>,
        all_drivers: impl IntoIterator<Item = Box<dyn SignWithFactorSourceDriver>>,
        signers_of_transaction: F,
    ) -> Result<Self>
    where
        F: Fn(TransactionIntent) -> Vec<HDPublicKey>,
    {
        //    let inputs: HashMap<IntentHash, Vec<HDPublicKey>> = transaction.into_iter().
        //    Self::new_batch_sign_transactions_grouping(inputs, )
        let instances_per_transaction: HashMap<IntentHash, Vec<HDPublicKey>> = HashMap::new();
        Self::new_batch_sign_transactions_grouping(
            instances_per_transaction,
            factor_sources,
            all_drivers,
        )
    }

    pub fn new_batch_sign_by_analyzing_transactions(
        transactions: Vec<TransactionIntent>,
        factor_sources: IndexSet<FactorSource>,
        all_drivers: impl IntoIterator<Item = Box<dyn SignWithFactorSourceDriver>>,
    ) -> Result<Self> {
        Self::new_batch_sign_by_analyzing_transactions_using(
            transactions,
            factor_sources,
            all_drivers,
            |t| todo!(),
        )
    }
}
