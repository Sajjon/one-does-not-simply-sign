use crate::prelude::*;

/// === FIA ===

pub struct FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    phantom_id: PhantomData<ID>,
    phantom_path: PhantomData<Path>,
    phantom_product: PhantomData<Product>,
}

/// Produce many signatures per transaction intent per FactorSource
pub type FIATransactionSigning = FactorInstanceAccumulator<IntentHash, HDPublicKey, HDSignature>;

/// Derives many public keys per FactorSource, e.g. used to `SecurityStructureOfFactorSources -> SecurityStructureOfFactorInstances`
pub type FIADeriveKeys = FactorInstanceAccumulator<DeriveKeyID, DerivationPath, HDPublicKey>;

/// ===== Public =====
impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    pub fn new(
        request: BatchUseFactorSourceRequest<ID, Path>,
        factor_sources: Vec<FactorSource>,
    ) -> Result<Self> {
        todo!()
    }

    pub async fn accumulate(&self) -> Result<BatchUseFactorSourceResponse<ID, Product>> {
        todo!()
    }
}

impl FIATransactionSigning {
    pub fn new_batch_sign_transactions(
        inputs: HashMap<FactorSourceID, HashMap<IntentHash, Vec<HDPublicKey>>>,
        factor_sources: Vec<FactorSource>,
    ) -> Result<Self> {
        Self::new(BatchUseFactorSourceRequest::new(inputs), factor_sources)
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

impl FIADeriveKeys {
    pub fn new_batch_derive_public_keys(
        inputs: HashMap<FactorSourceID, HashMap<DeriveKeyID, Vec<DerivationPath>>>,
        factor_sources: Vec<FactorSource>,
    ) -> Result<Self> {
        Self::new(BatchUseFactorSourceRequest::new(inputs), factor_sources)
    }
}
