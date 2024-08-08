use crate::prelude::*;

/// === BASIC TYPES ===
///

#[derive(Debug, Clone)]
pub struct DerivationPath;

#[derive(Debug, Clone)]
pub struct FactorSourceID;

#[derive(Debug, Clone)]
pub struct FactorSource;

#[derive(Debug, Clone, Hash)]
pub struct IntentHash;

#[derive(Debug, Clone, Hash)]
pub struct SecurityShieldID;

#[derive(Debug, Clone)]
pub struct PublicKey;

#[derive(Debug, Clone)]
pub struct Signature;

#[derive(Debug, Clone)]
pub struct HDPublicKey {
    derivation_path: DerivationPath,
    public_key: PublicKey,
}

#[derive(Debug, Clone)]
pub struct HDSignature {
    public_key: HDPublicKey,
    signature: Signature,
}

#[derive(Debug, Clone)]
pub struct TransactionIntent;

#[derive(Debug, Clone, Hash)]
pub enum DeriveKeyID {
    Single,
    SecurityShield(SecurityShieldID),
}

#[derive(Debug, Clone)]
pub enum AccountOrPersona {}

#[derive(Debug, Clone)]
pub enum Error {}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// === REQUEST TYPES ===
pub trait HasDerivationPath {
    fn derivation_path(&self) -> DerivationPath;
}
impl HasDerivationPath for DerivationPath {
    fn derivation_path(&self) -> DerivationPath {
        self.clone()
    }
}

impl HasDerivationPath for HDPublicKey {
    fn derivation_path(&self) -> DerivationPath {
        self.derivation_path.clone()
    }
}

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

pub type BatchDerivePublicKeysRequest = BatchUseFactorSourceRequest<DeriveKeyID, DerivationPath>;
pub type BatchSignTransactionsRequest = BatchUseFactorSourceRequest<IntentHash, HDPublicKey>;

/// === RESPONSE TYPES ===
pub trait HasHDPublicKey {
    fn hd_public_key(&self) -> HDPublicKey;
}
impl HasHDPublicKey for HDSignature {
    fn hd_public_key(&self) -> HDPublicKey {
        self.public_key.clone()
    }
}
impl HasHDPublicKey for HDPublicKey {
    fn hd_public_key(&self) -> HDPublicKey {
        self.clone()
    }
}

pub struct BatchUseFactorSourceResponse<ID, Product>
where
    ID: Hash,
    Product: HasHDPublicKey,
{
    outputs: HashMap<ID, Vec<Product>>,
}
pub type BatchDerivePublicKeysResponse = BatchUseFactorSourceResponse<DeriveKeyID, HDPublicKey>;
pub type BatchSignTransactionsResponse = BatchUseFactorSourceResponse<IntentHash, HDSignature>;

#[async_trait::async_trait]
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

#[async_trait::async_trait]
pub trait SignWithFactorSourceDriver:
    UseFactorSourceDriver<IntentHash, HDPublicKey, HDSignature>
{
    /// Produces many signatures for many entities from many factor sources for many transactions.
    async fn batch_sign_transactions(
        &self,
        request: BatchSignTransactionsRequest,
    ) -> Result<BatchSignTransactionsResponse>;
}

#[async_trait::async_trait]
impl<T: SignWithFactorSourceDriver + std::marker::Sync>
    UseFactorSourceDriver<IntentHash, HDPublicKey, HDSignature> for T
{
    async fn use_factor(
        &self,
        request: BatchUseFactorSourceRequest<IntentHash, HDPublicKey>,
    ) -> Result<BatchUseFactorSourceResponse<IntentHash, HDSignature>> {
        self.batch_sign_transactions(request).await
    }
}

#[async_trait::async_trait]
pub trait DeriveKeysWithFactorSourceDriver:
    UseFactorSourceDriver<(), DerivationPath, HDPublicKey>
{
    /// Derives many keys from many factor sources for many entities.
    async fn batch_derive_public_keys(
        &self,
        request: BatchDerivePublicKeysRequest,
    ) -> Result<BatchDerivePublicKeysResponse>;
}

#[async_trait::async_trait]
impl<T: DeriveKeysWithFactorSourceDriver + std::marker::Sync>
    UseFactorSourceDriver<DeriveKeyID, DerivationPath, HDPublicKey> for T
{
    async fn use_factor(
        &self,
        request: BatchDerivePublicKeysRequest,
    ) -> Result<BatchDerivePublicKeysResponse> {
        self.batch_derive_public_keys(request).await
    }
}
