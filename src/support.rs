use crate::prelude::*;

/// === BASIC TYPES ===
///

#[derive(Debug, Clone)]
pub struct DerivationPath;

#[derive(Debug, Clone)]
pub struct FactorSourceID;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FactorSourceKind {
    Device,
    Ledger,
}
impl FactorSourceKind {
    pub fn supports_parallelism(&self) -> bool {
        match self {
            Self::Device => true,
            Self::Ledger => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FactorSource {
    pub kind: FactorSourceKind,
}

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

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("InvalidFactorSourceKind")]
    InvalidFactorSourceKind,
}

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
