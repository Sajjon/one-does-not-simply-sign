use std::time::SystemTime;

use crate::prelude::*;
use itertools::Itertools;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct FactorSourceID {
    pub kind: FactorSourceKind,
    pub id: Uuid,
}
impl FactorSourceID {
    pub fn new(kind: FactorSourceKind) -> Self {
        Self {
            kind,
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
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
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<std::cmp::Ordering> {
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

impl FactorSource {
    fn sign(
        &self,
        _intent_hash: &IntentHash,
        _factor_instance: &FactorInstance,
    ) -> Signature {
        Signature
    }


}

#[repr(u32)]
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    std::hash::Hash,
    PartialOrd,
    Ord,
)]
pub enum FactorSourceKind {
    Ledger,
    Arculus,
    Yubikey,
    SecurityQuestions,
    OffDeviceMnemonic,
    Device,
}

pub trait IsFactorSource {
    fn kind() -> FactorSourceKind;
}
pub struct ArculusFactorSource;
impl IsFactorSource for ArculusFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::Arculus
    }
}
pub struct LedgerFactorSource;
impl IsFactorSource for LedgerFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::Ledger
    }
}
pub struct YubikeyFactorSource;
impl IsFactorSource for YubikeyFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::Yubikey
    }
}
pub struct SecurityQuestionsFactorSource;
impl IsFactorSource for SecurityQuestionsFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::SecurityQuestions
    }
}

pub struct OffDeviceMnemonicFactorSource;
impl IsFactorSource for OffDeviceMnemonicFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::OffDeviceMnemonic
    }
}
pub struct DeviceMnemonicFactorSource;
impl IsFactorSource for DeviceMnemonicFactorSource {
    fn kind() -> FactorSourceKind {
        FactorSourceKind::Device
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct FactorInstance {
    pub index: u32,
    pub factor_source_id: FactorSourceID,
}

pub trait FactorSourceReferencing {
    fn factor_source_id(&self) -> FactorSourceID;
}

impl FactorSourceReferencing for FactorInstance {
    fn factor_source_id(&self) -> FactorSourceID {
        self.factor_source_id
    }
}

impl FactorInstance {
    pub fn new(index: u32, factor_source_id: FactorSourceID) -> Self {
        Self {
            index,
            factor_source_id,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct OwnedFactorInstance {
    pub factor_instance: FactorInstance,
    pub owner: AccountAddressOrIdentityAddress,
}
impl OwnedFactorInstance {
    pub fn new(
        factor_instance: FactorInstance,
        owner: AccountAddressOrIdentityAddress,
    ) -> Self {
        Self {
            factor_instance,
            owner,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct Hash {
    id: Uuid,
}
impl Hash {
    pub fn generate() -> Self {
        Self { id: Uuid::new_v4() }
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

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct AccountAddressOrIdentityAddress {
    pub name: String,
    id: Uuid,
}
impl AccountAddressOrIdentityAddress {
    fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            id: Uuid::new_v4(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct Entity {
    pub address: AccountAddressOrIdentityAddress,
    pub security_state: EntitySecurityState,
}
impl Entity {
    fn new(
        name: impl AsRef<str>,
        security_state: impl Into<EntitySecurityState>,
    ) -> Self {
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
            EntitySecurityState::Unsecured(FactorInstance::new(
                index,
                factor_source_id,
            )),
        )
    }
}

impl From<&Entity> for OwnedMatrixOfFactorInstances {
    fn from(value: &Entity) -> Self {
        let matrix = match value.security_state.clone() {
            EntitySecurityState::Securified(matrix) => matrix.clone(),
            EntitySecurityState::Unsecured(instance) => {
                MatrixOfFactorInstances::from(instance)
            }
        };
        OwnedMatrixOfFactorInstances {
            address_of_owner: value.address.clone(),
            matrix,
        }
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
        let threshold_factors =
            threshold_factors.into_iter().collect_vec();
        assert!(threshold_factors.len() >= threshold as usize);
        Self {
            threshold_factors,
            threshold,
            override_factors: override_factors
                .into_iter()
                .collect_vec(),
        }
    }
    pub fn override_only(
        factors: impl IntoIterator<Item = FactorInstance>,
    ) -> Self {
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

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct OwnedMatrixOfFactorInstances {
    pub address_of_owner: AccountAddressOrIdentityAddress,
    pub matrix: MatrixOfFactorInstances,
}
impl OwnedMatrixOfFactorInstances {
    pub fn new(
        address_of_owner: AccountAddressOrIdentityAddress,
        matrix: MatrixOfFactorInstances,
    ) -> Self {
        Self {
            address_of_owner,
            matrix,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct IntentHash {
    hash: Hash,
}
impl IntentHash {
    pub fn generate() -> Self {
        Self {
            hash: Hash::generate(),
        }
    }
    pub fn new() -> Self {
        Self::generate()
    }
    pub fn hash(&self) -> Hash {
        self.hash.clone()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct TransactionIntent {
    pub intent_hash: IntentHash,
    pub entities_requiring_auth: Vec<Entity>, // should be a set but Sets are not `Hash`.
}
impl TransactionIntent {
    pub fn new(
        entities_requiring_auth: impl IntoIterator<Item = Entity>,
    ) -> Self {
        Self {
            intent_hash: IntentHash::generate(),
            entities_requiring_auth: entities_requiring_auth
                .into_iter()
                .collect_vec(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionsPayloads {
    pub intents: IndexMap<IntentHash, TransactionIntent>,
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct Signature;

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct SignatureByOwnedFactorForPayload {
    pub intent_hash: IntentHash,
    pub owned_factor_instance: OwnedFactorInstance,
    pub signature: Signature,
}

impl SignatureByOwnedFactorForPayload {
    pub fn new(
        intent_hash: IntentHash,
        owned_factor_instance: OwnedFactorInstance,
        signature: Signature,
    ) -> Self {
        Self {
            intent_hash,
            owned_factor_instance,
            signature,
        }
    }
    pub fn factor_source_id(&self) -> &FactorSourceID {
        &self.owned_factor_instance.factor_instance.factor_source_id
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

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct InvalidTransactionIfSkipped {
    pub intent_hash: IntentHash,
    pub entities_which_would_fail_auth:
        Vec<AccountAddressOrIdentityAddress>,
}
impl InvalidTransactionIfSkipped {
    pub fn new(
        intent_hash: IntentHash,
        entities_which_would_fail_auth: Vec<
            AccountAddressOrIdentityAddress,
        >,
    ) -> Self {
        Self {
            intent_hash,
            entities_which_would_fail_auth,
        }
    }
}
