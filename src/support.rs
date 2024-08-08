use crate::prelude::*;

/// === BASIC TYPES ===
///

#[derive(Debug, Clone)]
pub struct DerivationPath;

#[derive(Debug, Clone)]
pub struct FactorSourceID;

#[derive(Debug, Clone)]
pub enum FactorSourceKind {
    Device,
    Ledger,
}

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
