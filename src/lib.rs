use std::{collections::HashMap, hash::Hash, marker::PhantomData};

/// === BASIC TYPES ===
pub struct DerivationPath;
pub struct FactorSourceID;
pub struct FactorSource;

#[derive(Hash)]
pub struct IntentHash;

pub struct SecurityShieldID;
pub struct PublicKey;
pub struct Signature;
pub struct HDPublicKey {
    derivation_path: DerivationPath,
    public_key: PublicKey,
}

pub struct HDSignature {
    public_key: HDPublicKey,
    signature: Signature,
}

pub struct TransactionIntent;
pub enum AccountOrPersona {}

pub enum Error {}
pub type Result<T, E = Error> = std::result::Result<T, E>;

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

/// Derives many public keys per FactorSource, e.g. used to `SecurityStructureOfFactorSources -> SecurityStructureOfFactorInstances` (letting ID be `SecurityShieldID`.)
pub type FIADeriveKeys<ID> = FactorInstanceAccumulator<ID, DerivationPath, HDPublicKey>;

/// === REQUEST TYPES ===
pub trait HasDerivationPath {
    fn derivation_path(&self) -> DerivationPath;
}
impl HasDerivationPath for DerivationPath {
    fn derivation_path(&self) -> DerivationPath {
        self
    }
}

impl HasDerivationPath for HDPublicKey {
    fn derivation_path(&self) -> DerivationPath {
        self.derivation_path
    }
}

pub struct BatchUseFactorSourceRequest<ID, Path>
where
    ID: Hash,
    Path: HasDerivationPath,
{
    inputs: HashMap<FactorSourceID, HashMap<ID, Vec<Path>>>,
}

pub type BatchDerivePublicKeysRequest<ID> = BatchUseFactorSourceRequest<ID, DerivationPath>;
pub type BatchSignTransactionsRequest = BatchUseFactorSourceRequest<IntentHash, HDPublicKey>;

/// === RESPONSE TYPES ===
pub trait HasHDPublicKey {
    fn hd_public_key(&self) -> HDPublicKey;
}
impl HasHDPublicKey for HDSignature {
    fn hd_public_key(&self) -> HDPublicKey {
        self.public_key
    }
}
impl HasHDPublicKey for HDPublicKey {
    fn hd_public_key(&self) -> HDPublicKey {
        self
    }
}

pub struct BatchUseFactorSourceResponse<ID, Product>
where
    ID: Hash,
    Product: HasHDPublicKey,
{
    outputs: HashMap<ID, Vec<Product>>,
}
pub type BatchDerivePublicKeysResponse<ID> = BatchUseFactorSourceResponse<ID, HDPublicKey>;
pub type BatchSignTransactionsResponse = BatchUseFactorSourceResponse<IntentHash, HDSignature>;

pub trait UseFactorSourceDriver<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
{
    async fn use_factor(
        &self,
        request: BatchUseFactorSourceRequest<ID, Path>,
    ) -> Result<BatchUseFactorSourceResponse<ID, Product>>;
}

pub trait SignWithFactorSourceDriver:
    UseFactorSourceDriver<IntentHash, HDPublicKey, HDSignature>
{
    /// Produces many signatures for many entities from many factor sources for many transactions.
    async fn batch_sign_transactions(
        &self,
        request: BatchSignTransactionsRequest,
    ) -> Result<BatchSignTransactionsResponse>;
}
impl<T: SignWithFactorSourceDriver> UseFactorSourceDriver<IntentHash, HDPublicKey, HDSignature>
    for T
{
    async fn use_factor(
        &self,
        request: BatchUseFactorSourceRequest<ID, Path>,
    ) -> Result<BatchUseFactorSourceResponse<ID, Product>> {
        self.batch_sign_transactions(request).await
    }
}

// pub trait DeriveKeysWithFactorSourceDriver:
//     UseFactorSourceDriver<(), DerivationPath, HDPublicKey>
// {
//     /// Derives many keys from many factor sources for many entities.
//     async fn batch_derive_public_keys(
//         &self,
//         request: BatchDerivePublicKeysRequest,
//     ) -> Result<BatchDerivePublicKeysResponse>;
// }

// impl<T: DeriveKeysWithFactorSourceDriver> UseFactorSourceDriver<(), DerivationPath, HDPublicKey>
//     for T
// {
//     async fn use_factor(
//         &self,
//         request: BatchDerivePublicKeysRequest,
//     ) -> Result<BatchDerivePublicKeysResponse> {
//         self.batch_derive_public_keys(request).await
//     }
// }

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

trait TyEq {}

impl<T: ?Sized> TyEq for (*const T, *const T) {}

impl<ID, Path, Product> FactorInstanceAccumulator<ID, Path, Product>
where
    ID: Hash,
    Path: HasDerivationPath,
    Product: HasHDPublicKey,
    (*const ID, *const IntentHash): TyEq,    // ID == IntentHash
    (*const Path, *const HDPublicKey): TyEq, // Path == HDPublicKey
    (*const Product, *const HDSignature): TyEq, // Product == HDSignature
                                             // https://github.com/rust-lang/rust/issues/20041
{
    pub fn new_batch_sign_transactions(
        inputs: HashMap<FactorSourceID, HashMap<IntentHash, Vec<HDPublicKey>>>,
        factor_sources: Vec<FactorSource>,
    ) -> Result<Self> {
        Self::new(BatchSignTransactionsRequest::new(inputs), factor_sources)
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

impl FactorInstanceAccumulator
where
    (*const ID, *const SecurityShieldID): TyEq, // ID == SecurityShieldID
    (*const Path, *const DerivationPath): TyEq, // Path == DerivationPath
    (*const Product, *const HDPublicKey): TyEq, // Product == HDPublicKey
                                                // https://github.com/rust-lang/rust/issues/20041
{
}
