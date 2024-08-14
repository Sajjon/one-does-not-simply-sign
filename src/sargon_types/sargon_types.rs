use crate::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, std::hash::Hash)]
pub struct DerivationPath(u32);

#[derive(Clone, Copy, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{kind}:{id}")]
pub struct FactorSourceID {
    pub kind: FactorSourceKind,
    pub id: Uuid,
}

impl FactorSourceID {
    fn with_details(kind: FactorSourceKind, id: Uuid) -> Self {
        Self { kind, id }
    }
    pub fn new(kind: FactorSourceKind) -> Self {
        Self::with_details(kind, Uuid::new_v4())
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.id.as_bytes().to_vec()
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash, PartialOrd, Ord, strum::Display)]
pub enum FactorSourceKind {
    Ledger,
    Arculus,
    Yubikey,
    SecurityQuestions,
    OffDeviceMnemonic,
    Device,
}

impl FactorSourceKind {
    pub fn supports_parallelism(&self) -> bool {
        match self {
            Self::Device => true,
            Self::Ledger
            | Self::Arculus
            | Self::Yubikey
            | Self::SecurityQuestions
            | Self::OffDeviceMnemonic => false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{:?}", id)]
pub struct FactorSource {
    pub last_used: SystemTime,
    pub id: FactorSourceID,
}

impl FactorSource {
    pub fn kind(&self) -> FactorSourceKind {
        self.id.kind
    }
    pub fn new(kind: FactorSourceKind) -> Self {
        Self {
            id: FactorSourceID::new(kind),
            last_used: SystemTime::now(),
        }
    }
    pub fn arculus() -> Self {
        Self::new(FactorSourceKind::Arculus)
    }
    pub fn ledger() -> Self {
        Self::new(FactorSourceKind::Ledger)
    }
    pub fn device() -> Self {
        Self::new(FactorSourceKind::Device)
    }
    pub fn yubikey() -> Self {
        Self::new(FactorSourceKind::Yubikey)
    }
    pub fn off_device() -> Self {
        Self::new(FactorSourceKind::OffDeviceMnemonic)
    }
    pub fn security_question() -> Self {
        Self::new(FactorSourceKind::SecurityQuestions)
    }
}

#[derive(Debug, Clone, Hash)]
pub struct SecurityShieldID;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublicKey;

#[derive(Debug, Clone)]
pub struct Signature;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HDPublicKey {
    factor_source_id: FactorSourceID,
    derivation_path: DerivationPath,
    public_key: PublicKey,
}

#[derive(Debug, Clone)]
pub struct HDSignature {
    public_key: HDPublicKey,
    signature: Signature,
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct IntentHash(Uuid);
impl IntentHash {
    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct TransactionIntent {
    pub intent_hash: IntentHash,
    pub entities_requiring_auth: Vec<Entity>, // should be a set but Sets are not `Hash`.
}
impl TransactionIntent {
    pub fn new(entities_requiring_auth: impl IntoIterator<Item = Entity>) -> Self {
        Self {
            intent_hash: IntentHash::generate(),
            entities_requiring_auth: entities_requiring_auth.into_iter().collect_vec(),
        }
    }
    pub fn just(entity: Entity) -> Self {
        Self::new([entity])
    }
}

#[derive(Debug, Clone, Hash)]
pub enum DeriveKeyID {
    Single,
    SecurityShield(SecurityShieldID),
}

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{name}")]
pub struct AccountAddressOrIdentityAddress {
    pub name: String,
    id: Uuid,
}
impl AccountAddressOrIdentityAddress {
    fn with_details(name: impl AsRef<str>, id: Uuid) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            id,
        }
    }
    pub fn new(name: impl AsRef<str>) -> Self {
        Self::with_details(name, Uuid::new_v4())
    }
}

/// Account or Persona
#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct Entity {
    pub address: AccountAddressOrIdentityAddress,
    pub security_state: EntitySecurityState,
}

impl Entity {
    fn new(name: impl AsRef<str>, security_state: impl Into<EntitySecurityState>) -> Self {
        Self {
            address: AccountAddressOrIdentityAddress::new(name),
            security_state: security_state.into(),
        }
    }

    pub fn securified(
        index: u32,
        name: impl AsRef<str>,
        make_matrix: fn(u32) -> MatrixOfFactorInstances,
    ) -> Self {
        Self::new(name, make_matrix(index))
    }

    pub fn unsecurified(
        index: u32,
        name: impl AsRef<str>,
        factor_source_id: FactorSourceID,
    ) -> Self {
        Self::new(
            name,
            EntitySecurityState::Unsecured(FactorInstance::new(index, factor_source_id)),
        )
    }

    pub fn unsecurified_anonymous(factor_source_id: FactorSourceID) -> Self {
        Self::unsecurified(0, "Anonymous", factor_source_id)
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("InvalidFactorSourceKind")]
    InvalidFactorSourceKind,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub trait HasDerivationPath: Hash {
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

pub trait HasHDPublicKey: Clone {
    fn hd_public_key(&self) -> HDPublicKey;
    fn derivation_path(&self) -> DerivationPath {
        self.hd_public_key().derivation_path.clone()
    }
    fn factor_source_id(&self) -> FactorSourceID {
        self.hd_public_key().factor_source_id.clone()
    }
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

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct MatrixOfFactorInstances {
    pub threshold_factors: Vec<FactorInstance>,
    pub threshold: u8,
    pub override_factors: Vec<FactorInstance>,
}

impl MatrixOfFactorInstances {
    /// Panics if threshold > threshold_factor.len()
    pub fn new(
        threshold_factors: impl IntoIterator<Item = FactorInstance>,
        threshold: u8,
        override_factors: impl IntoIterator<Item = FactorInstance>,
    ) -> Self {
        let threshold_factors = threshold_factors.into_iter().collect_vec();
        assert!(threshold_factors.len() >= threshold as usize);
        Self {
            threshold_factors,
            threshold,
            override_factors: override_factors.into_iter().collect_vec(),
        }
    }
    pub fn override_only(factors: impl IntoIterator<Item = FactorInstance>) -> Self {
        Self::new([], 0, factors)
    }
    pub fn single_override(factor: FactorInstance) -> Self {
        Self::override_only([factor])
    }
    pub fn threshold_only(
        factors: impl IntoIterator<Item = FactorInstance>,
        threshold: u8,
    ) -> Self {
        Self::new(factors, threshold, [])
    }
    pub fn single_threshold(factor: FactorInstance) -> Self {
        Self::threshold_only([factor], 1)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct FactorInstance {
    pub derivation_path: DerivationPath,
    pub factor_source_id: FactorSourceID,
}

impl FactorInstance {
    pub fn new(index: u32, factor_source_id: FactorSourceID) -> Self {
        Self {
            derivation_path: DerivationPath(index),
            factor_source_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum EntitySecurityState {
    Unsecured(FactorInstance),
    Securified(MatrixOfFactorInstances),
}
impl EntitySecurityState {
    pub fn all_factor_instances(&self) -> IndexSet<FactorInstance> {
        match self {
            Self::Securified(matrix) => {
                let mut set = IndexSet::new();
                set.extend(matrix.threshold_factors.clone());
                set.extend(matrix.override_factors.clone());
                set
            }
            Self::Unsecured(fi) => IndexSet::from_iter([fi.clone()]),
        }
    }
}

impl From<MatrixOfFactorInstances> for EntitySecurityState {
    fn from(value: MatrixOfFactorInstances) -> Self {
        Self::Securified(value)
    }
}
