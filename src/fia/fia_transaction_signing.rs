use crate::prelude::*;

/// Produce many signatures per transaction intent per FactorSource
pub type FiaTransactionSigning = FactorInstanceAccumulator<IntentHash, HDPublicKey, HDSignature>;

impl FiaTransactionSigning {
    pub fn new_batch_sign_transactions(
        inputs: HashMap<FactorSourceID, HashMap<IntentHash, Vec<HDPublicKey>>>,
        factor_sources: Vec<FactorSource>,
    ) -> Result<Self> {
        //        Self::new(BatchUseFactorSourceRequest::new(inputs), factor_sources)
        todo!()
    }

    pub fn new_batch_sign_transactions_grouping(
        instances_per_transaction: HashMap<IntentHash, Vec<HDPublicKey>>,
        factor_sources: Vec<FactorSource>,
    ) -> Result<Self> {
        // let inputs = ...
        // Self::new_batch_sign_transactions(inputs, factor_sources)
        todo!()
    }

    pub fn new_batch_sign_by_analyzing_transactions_using<F>(
        transactions: Vec<TransactionIntent>,
        entities: Vec<AccountOrPersona>,
        factor_sources: Vec<FactorSource>,
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
        entities: Vec<AccountOrPersona>,
        factor_sources: Vec<FactorSource>,
    ) -> Result<Self> {
        //    let inputs = ...
        //    Self::new_batch_sign_transactions(inputs)
        todo!()
    }
}
