use crate::prelude::*;

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

    pub fn sample_third() -> Self {
        Self::with_details(FactorSourceKind::Arculus, Uuid::from_bytes([0xaa; 16]))
    }

    pub fn sample_fourth() -> Self {
        Self::with_details(
            FactorSourceKind::SecurityQuestions,
            Uuid::from_bytes([0x5e; 16]),
        )
    }
}

impl HasSampleValues for FactorSourceID {
    fn sample() -> Self {
        Self::with_details(FactorSourceKind::Device, Uuid::from_bytes([0xde; 16]))
    }
    fn sample_other() -> Self {
        Self::with_details(FactorSourceKind::Ledger, Uuid::from_bytes([0x1e; 16]))
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

impl PartialOrd for FactorSource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for FactorSource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind().cmp(&other.kind()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.last_used.cmp(&other.last_used) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        core::cmp::Ordering::Equal
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

impl HasSampleValues for FactorSourceKind {
    fn sample() -> Self {
        FactorSourceKind::Device
    }
    fn sample_other() -> Self {
        FactorSourceKind::Ledger
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct FactorInstance {
    pub index: u32, // actually `DerivationPath`...
    pub factor_source_id: FactorSourceID,
}

impl FactorInstance {
    pub fn new(index: u32, factor_source_id: FactorSourceID) -> Self {
        Self {
            index,
            factor_source_id,
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        [
            self.index.to_be_bytes().to_vec(),
            self.factor_source_id.to_bytes(),
        ]
        .concat()
    }
}

impl HasSampleValues for FactorInstance {
    fn sample() -> Self {
        Self::new(0, FactorSourceID::sample())
    }
    fn sample_other() -> Self {
        Self::new(1, FactorSourceID::sample_other())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct Hash {
    id: Uuid,
}
impl Hash {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.id.as_bytes().to_vec()
    }
    fn new(id: Uuid) -> Self {
        Self { id }
    }
    pub fn generate() -> Self {
        Self::new(Uuid::new_v4())
    }
    pub fn sample_third() -> Self {
        Self::new(Uuid::from_bytes([0x11; 16]))
    }
}
impl HasSampleValues for Hash {
    fn sample() -> Self {
        Self::new(Uuid::from_bytes([0xde; 16]))
    }
    fn sample_other() -> Self {
        Self::new(Uuid::from_bytes([0xab; 16]))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct AccountAddress;

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct IdentityAddress;

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
impl HasSampleValues for AccountAddressOrIdentityAddress {
    fn sample() -> Self {
        Self::with_details("Alice", Uuid::from_bytes([0xac; 16]))
    }
    fn sample_other() -> Self {
        Self::with_details("Bob", Uuid::from_bytes([0xc0; 16]))
    }
}

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

/// For unsecurified entities we map single factor -> single threshold factor.
/// Which is used by ROLA.
impl From<FactorInstance> for MatrixOfFactorInstances {
    fn from(value: FactorInstance) -> Self {
        Self {
            threshold: 1,
            threshold_factors: vec![value],
            override_factors: Vec::new(),
        }
    }
}

pub trait HasSampleValues {
    fn sample() -> Self;
    fn sample_other() -> Self;
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash, Getters)]
pub struct IntentHash {
    hash: Hash,
}

impl IntentHash {
    fn new(hash: Hash) -> Self {
        Self { hash }
    }
    pub fn generate() -> Self {
        Self::new(Hash::generate())
    }
    pub fn sample_third() -> Self {
        Self::new(Hash::sample_third())
    }
}

impl HasSampleValues for IntentHash {
    fn sample() -> Self {
        Self::new(Hash::sample())
    }
    fn sample_other() -> Self {
        Self::new(Hash::sample_other())
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
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct Signature(String);
impl HasSampleValues for Signature {
    fn sample() -> Self {
        Self("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef".to_owned())
    }
    fn sample_other() -> Self {
        Self("fadecafefadecafefadecafefadecafefadecafefadecafefadecafefadecafefadecafefadecafefadecafefadecafefadecafefadecafefadecafefadecafe".to_owned())
    }
}
impl Signature {
    /// Emulates the signing of `intent_hash` with `factor_instance` - in a
    /// deterministic manner.
    pub fn produced_by(
        intent_hash: IntentHash,
        factor_instance: impl Into<FactorInstance>,
    ) -> Self {
        let factor_instance = factor_instance.into();

        let intent_hash_bytes = intent_hash.hash().to_bytes();
        let factor_instance_bytes = factor_instance.to_bytes();
        let input_bytes = [intent_hash_bytes, factor_instance_bytes].concat();
        let hash = sha256::digest(input_bytes);
        Self(hash)
    }

    /// Emulates signing using `input`.
    pub fn produced_by_input(input: &HDSignatureInput) -> Self {
        Self::produced_by(
            input.intent_hash.clone(),
            input.owned_factor_instance.clone(),
        )
    }
}

pub type Result<T, E = CommonError> = std::result::Result<T, E>;

#[derive(thiserror::Error, Clone, Debug)]
pub enum CommonError {
    #[error("Unknown factor source")]
    UnknownFactorSource,

    #[error("Failed")]
    Failure,
}
