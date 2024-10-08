use std::marker::PhantomData;

use crate::prelude::*;

use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::ops::AddAssign;
use std::sync::Mutex;

/// An UNSAFE IDStepper, which `next` returns the consecutive next ID,
/// should only be used by tests and sample value creation.
pub struct IDStepper<T: From<Uuid>> {
    ctr: Arc<Mutex<u64>>,
    phantom: PhantomData<T>,
}
pub type UuidStepper = IDStepper<Uuid>;

impl<T: From<Uuid>> IDStepper<T> {
    pub fn starting_at(ctr: u64) -> Self {
        Self {
            ctr: Arc::new(Mutex::new(ctr)),
            phantom: PhantomData,
        }
    }

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::starting_at(0)
    }

    /// ONLY Use this in a test or when creating sample (preview) values.
    ///
    /// # Safety
    /// This is completely unsafe, it does not generate a random UUID, it creates
    /// the consecutive "next" ID.
    pub fn _next(&self) -> T {
        let n = Uuid::from_u64_pair(0, **self.ctr.lock().unwrap().borrow());
        self.ctr.lock().unwrap().borrow_mut().add_assign(1);
        n.into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, std::hash::Hash, derive_more::Display, derive_more::Debug)]
#[display("{kind}:{id}")]
#[debug("{}", self.to_string())]
pub struct FactorSourceIDFromHash {
    pub kind: FactorSourceKind,
    pub id: Uuid,
}

impl FactorSourceIDFromHash {
    fn with_details(kind: FactorSourceKind, id: Uuid) -> Self {
        Self { kind, id }
    }
    pub fn new(kind: FactorSourceKind) -> Self {
        Self::with_details(kind, IDStepper::next())
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

impl HasSampleValues for FactorSourceIDFromHash {
    fn sample() -> Self {
        Self::with_details(FactorSourceKind::Device, Uuid::from_bytes([0xde; 16]))
    }
    fn sample_other() -> Self {
        Self::with_details(FactorSourceKind::Ledger, Uuid::from_bytes([0x1e; 16]))
    }
}

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{:#?}", id)]
pub struct HDFactorSource {
    pub last_used: SystemTime,
    id: FactorSourceIDFromHash,
}

impl HDFactorSource {
    pub fn factor_source_id(&self) -> FactorSourceIDFromHash {
        self.id
    }
    pub fn factor_source_kind(&self) -> FactorSourceKind {
        self.id.kind
    }
    pub fn new(kind: FactorSourceKind) -> Self {
        Self {
            id: FactorSourceIDFromHash::new(kind),
            last_used: SystemTime::UNIX_EPOCH,
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

impl PartialOrd for HDFactorSource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HDFactorSource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.factor_source_kind().cmp(&other.factor_source_kind()) {
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

pub trait Just<Item> {
    fn just(item: Item) -> Self;
}
impl<T: std::hash::Hash + Eq> Just<T> for IndexSet<T> {
    fn just(item: T) -> Self {
        Self::from_iter([item])
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

pub type HDPathValue = u32;

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, derive_more::Display, derive_more::Debug,
)]
#[display("{value}")]
#[debug("{value}")]
pub struct HDPathComponent {
    pub value: HDPathValue,
}
pub const BIP32_SECURIFIED_HALF: u32 = 0x4000_0000;
pub(crate) const BIP32_HARDENED: u32 = 0x8000_0000;

impl HDPathComponent {
    pub fn non_hardened(value: HDPathValue) -> Self {
        assert!(
            value < BIP32_HARDENED,
            "Passed value was hardened, expected it to not be."
        );
        Self { value }
    }
    pub fn securified(value: HDPathValue) -> Self {
        Self::non_hardened(value + BIP32_SECURIFIED_HALF)
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}
impl HasSampleValues for HDPathComponent {
    fn sample() -> Self {
        Self::non_hardened(0)
    }
    fn sample_other() -> Self {
        Self::non_hardened(1)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, derive_more::Debug)]
pub enum CAP26KeyKind {
    #[display("tx")]
    #[debug("tx")]
    T9n,

    #[display("rola")]
    #[debug("rola")]
    Rola,
}
impl CAP26KeyKind {
    fn discriminant(&self) -> u8 {
        core::intrinsics::discriminant_value(self)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, derive_more::Debug)]
pub enum NetworkID {
    #[display("Mainnet")]
    #[debug("0")]
    Mainnet,

    #[display("Stokenet")]
    #[debug("1")]
    Stokenet,
}

impl NetworkID {
    fn discriminant(&self) -> u8 {
        core::intrinsics::discriminant_value(self)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, derive_more::Display, derive_more::Debug)]
pub enum CAP26EntityKind {
    #[display("Account")]
    #[debug("A")]
    Account,

    #[display("Identity")]
    #[debug("I")]
    Identity,
}

impl CAP26EntityKind {
    fn discriminant(&self) -> u8 {
        core::intrinsics::discriminant_value(self)
    }
}

#[derive(Clone, PartialEq, Eq, Hash, derive_more::Display, derive_more::Debug)]
#[display("{}/{}/{}/{}", network_id, entity_kind, key_kind, index)]
#[debug("{:?}/{:?}/{:?}/{:?}", network_id, entity_kind, key_kind, index)]
pub struct DerivationPath {
    pub network_id: NetworkID,
    pub entity_kind: CAP26EntityKind,
    pub key_kind: CAP26KeyKind,
    pub index: HDPathComponent,
}

impl DerivationPath {
    pub fn new(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: HDPathComponent,
    ) -> Self {
        Self {
            network_id,
            entity_kind,
            key_kind,
            index,
        }
    }
    pub fn at(
        network_id: NetworkID,
        entity_kind: CAP26EntityKind,
        key_kind: CAP26KeyKind,
        index: HDPathValue,
    ) -> Self {
        Self::new(
            network_id,
            entity_kind,
            key_kind,
            HDPathComponent::non_hardened(index),
        )
    }
    pub fn account_tx(network_id: NetworkID, index: HDPathComponent) -> Self {
        Self::new(
            network_id,
            CAP26EntityKind::Account,
            CAP26KeyKind::T9n,
            index,
        )
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.push(self.network_id.discriminant());
        vec.push(self.entity_kind.discriminant());
        vec.push(self.key_kind.discriminant());
        vec.extend(self.index.to_bytes());
        vec
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PublicKey {
    /// this emulates the mnemonic
    factor_source_id: FactorSourceIDFromHash,
}
impl PublicKey {
    pub fn new(factor_source_id: FactorSourceIDFromHash) -> Self {
        Self { factor_source_id }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        self.factor_source_id.to_bytes()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HierarchicalDeterministicPublicKey {
    /// The expected public key of the private key derived at `derivationPath`
    pub public_key: PublicKey,

    /// The HD derivation path for the key pair which produces virtual badges (signatures).
    pub derivation_path: DerivationPath,
}
impl HierarchicalDeterministicPublicKey {
    pub fn new(derivation_path: DerivationPath, public_key: PublicKey) -> Self {
        Self {
            derivation_path,
            public_key,
        }
    }

    pub fn mocked_with(
        derivation_path: DerivationPath,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> Self {
        Self::new(derivation_path, PublicKey::new(*factor_source_id))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [self.public_key.to_bytes(), self.derivation_path.to_bytes()].concat()
    }
}

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{}", self.debug_str())]
pub struct HierarchicalDeterministicFactorInstance {
    pub factor_source_id: FactorSourceIDFromHash,
    pub public_key: HierarchicalDeterministicPublicKey,
}

impl HierarchicalDeterministicFactorInstance {
    #[allow(unused)]
    fn debug_str(&self) -> String {
        format!(
            "factor_source_id: {:#?}, derivation_path: {:#?}",
            self.factor_source_id, self.public_key.derivation_path
        )
    }

    pub fn new(
        public_key: HierarchicalDeterministicPublicKey,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        Self {
            public_key,
            factor_source_id,
        }
    }

    pub fn derivation_path(&self) -> DerivationPath {
        self.public_key.derivation_path.clone()
    }

    pub fn mocked_with(
        derivation_path: DerivationPath,
        factor_source_id: &FactorSourceIDFromHash,
    ) -> Self {
        Self::new(
            HierarchicalDeterministicPublicKey::mocked_with(derivation_path, factor_source_id),
            *factor_source_id,
        )
    }

    pub fn tx_on_network(
        entity_kind: CAP26EntityKind,
        network_id: NetworkID,
        index: HDPathComponent,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        let derivation_path =
            DerivationPath::new(network_id, entity_kind, CAP26KeyKind::T9n, index);
        let public_key = PublicKey::new(factor_source_id);
        let hd_public_key = HierarchicalDeterministicPublicKey::new(derivation_path, public_key);
        Self::new(hd_public_key, factor_source_id)
    }

    pub fn mainnet_tx(
        entity_kind: CAP26EntityKind,
        index: HDPathComponent,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        Self::tx_on_network(entity_kind, NetworkID::Mainnet, index, factor_source_id)
    }

    pub fn mainnet_tx_account(
        index: HDPathComponent,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        Self::mainnet_tx(CAP26EntityKind::Account, index, factor_source_id)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        [self.public_key.to_bytes(), self.factor_source_id.to_bytes()].concat()
    }
}

impl HasSampleValues for HierarchicalDeterministicFactorInstance {
    fn sample() -> Self {
        Self::mainnet_tx_account(HDPathComponent::sample(), FactorSourceIDFromHash::sample())
    }
    fn sample_other() -> Self {
        Self::mainnet_tx_account(
            HDPathComponent::sample_other(),
            FactorSourceIDFromHash::sample_other(),
        )
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
    Unsecured(HierarchicalDeterministicFactorInstance),
    Securified(MatrixOfFactorInstances),
}
impl EntitySecurityState {
    pub fn all_factor_instances(&self) -> IndexSet<HierarchicalDeterministicFactorInstance> {
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

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash, derive_more::Display)]
#[display("{name}")]
pub struct AbstractAddress<T: EntityKindSpecifier> {
    phantom: PhantomData<T>,
    pub name: String,
}
impl<T: EntityKindSpecifier> From<String> for AbstractAddress<T> {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
impl<T: EntityKindSpecifier> AbstractAddress<T> {
    pub fn entity_kind() -> CAP26EntityKind {
        T::entity_kind()
    }

    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            phantom: PhantomData,
            name: name.as_ref().to_owned(),
        }
    }
}
impl<T: EntityKindSpecifier> HasSampleValues for AbstractAddress<T> {
    fn sample() -> Self {
        Self::new("Alice")
    }
    fn sample_other() -> Self {
        Self::new("Bob")
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct AccountAddressTag;

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct IdentityAddressTag;

pub trait EntityKindSpecifier {
    fn entity_kind() -> CAP26EntityKind;
}
impl EntityKindSpecifier for AccountAddressTag {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Account
    }
}
impl EntityKindSpecifier for IdentityAddressTag {
    fn entity_kind() -> CAP26EntityKind {
        CAP26EntityKind::Identity
    }
}

impl<T: EntityKindSpecifier> EntityKindSpecifier for AbstractAddress<T> {
    fn entity_kind() -> CAP26EntityKind {
        T::entity_kind()
    }
}

pub type AccountAddress = AbstractAddress<AccountAddressTag>;
pub type IdentityAddress = AbstractAddress<IdentityAddressTag>;

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Display)]
pub enum AddressOfAccountOrPersona {
    #[display("acco_{_0}")]
    Account(AccountAddress),
    #[display("ident_{_0}")]
    Identity(IdentityAddress),
}
impl std::fmt::Debug for AddressOfAccountOrPersona {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}
impl HasSampleValues for AddressOfAccountOrPersona {
    fn sample() -> Self {
        Self::Account(AccountAddress::sample())
    }
    fn sample_other() -> Self {
        Self::Identity(IdentityAddress::sample())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub enum AccountOrPersona {
    AccountEntity(Account),
    PersonaEntity(Persona),
}

pub trait IsEntity: Into<AccountOrPersona> + Clone {
    type Address: Clone + Into<AddressOfAccountOrPersona> + EntityKindSpecifier;

    fn new(name: impl AsRef<str>, security_state: impl Into<EntitySecurityState>) -> Self;

    fn entity_address(&self) -> Self::Address;
    fn kind() -> CAP26EntityKind {
        Self::Address::entity_kind()
    }
    fn security_state(&self) -> EntitySecurityState;
    fn address(&self) -> AddressOfAccountOrPersona {
        self.entity_address().clone().into()
    }
    fn e0() -> Self;
    fn e1() -> Self;
    fn e2() -> Self;
    fn e3() -> Self;
    fn e4() -> Self;
    fn e5() -> Self;
    fn e6() -> Self;
    fn e7() -> Self;

    fn securified_mainnet(
        index: HDPathComponent,
        name: impl AsRef<str>,
        make_matrix: fn(HDPathComponent) -> MatrixOfFactorInstances,
    ) -> Self {
        Self::new(name, make_matrix(index))
    }

    fn unsecurified_mainnet(
        index: u32,
        name: impl AsRef<str>,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        Self::new(
            name,
            EntitySecurityState::Unsecured(HierarchicalDeterministicFactorInstance::mainnet_tx(
                Self::kind(),
                HDPathComponent::non_hardened(index),
                factor_source_id,
            )),
        )
    }
}

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{}", self.address())]
pub struct AbstractEntity<A: Clone + Into<AddressOfAccountOrPersona> + EntityKindSpecifier> {
    address: A,
    pub security_state: EntitySecurityState,
}
pub type Account = AbstractEntity<AccountAddress>;
impl IsEntity for Account {
    fn new(name: impl AsRef<str>, security_state: impl Into<EntitySecurityState>) -> Self {
        Self {
            address: AccountAddress::from(name.as_ref().to_owned()),
            security_state: security_state.into(),
        }
    }
    type Address = AccountAddress;
    fn security_state(&self) -> EntitySecurityState {
        self.security_state.clone()
    }
    fn entity_address(&self) -> Self::Address {
        self.address.clone()
    }
    fn e0() -> Self {
        Self::a0()
    }
    fn e1() -> Self {
        Self::a1()
    }
    fn e2() -> Self {
        Self::a2()
    }
    fn e3() -> Self {
        Self::a3()
    }
    fn e4() -> Self {
        Self::a4()
    }
    fn e5() -> Self {
        Self::a5()
    }
    fn e6() -> Self {
        Self::a6()
    }
    fn e7() -> Self {
        Self::a7()
    }
}

pub type Persona = AbstractEntity<IdentityAddress>;
impl IsEntity for Persona {
    fn new(name: impl AsRef<str>, security_state: impl Into<EntitySecurityState>) -> Self {
        Self {
            address: IdentityAddress::from(name.as_ref().to_owned()),
            security_state: security_state.into(),
        }
    }
    type Address = IdentityAddress;
    fn security_state(&self) -> EntitySecurityState {
        self.security_state.clone()
    }
    fn entity_address(&self) -> Self::Address {
        self.address.clone()
    }
    fn e0() -> Self {
        Self::p0()
    }
    fn e1() -> Self {
        Self::p1()
    }
    fn e2() -> Self {
        Self::p2()
    }
    fn e3() -> Self {
        Self::p3()
    }
    fn e4() -> Self {
        Self::p4()
    }
    fn e5() -> Self {
        Self::p5()
    }
    fn e6() -> Self {
        Self::p6()
    }
    fn e7() -> Self {
        Self::p7()
    }
}

impl<T: Clone + Into<AddressOfAccountOrPersona> + EntityKindSpecifier> EntityKindSpecifier
    for AbstractEntity<T>
{
    fn entity_kind() -> CAP26EntityKind {
        T::entity_kind()
    }
}

impl<T: Clone + Into<AddressOfAccountOrPersona> + EntityKindSpecifier> AbstractEntity<T> {
    pub fn address(&self) -> AddressOfAccountOrPersona {
        self.address.clone().into()
    }
}

impl From<Account> for AccountOrPersona {
    fn from(value: Account) -> Self {
        Self::AccountEntity(value)
    }
}

impl From<Persona> for AccountOrPersona {
    fn from(value: Persona) -> Self {
        Self::PersonaEntity(value)
    }
}

impl From<AccountAddress> for AddressOfAccountOrPersona {
    fn from(value: AccountAddress) -> Self {
        Self::Account(value)
    }
}

impl From<IdentityAddress> for AddressOfAccountOrPersona {
    fn from(value: IdentityAddress) -> Self {
        Self::Identity(value)
    }
}

impl HasSampleValues for Account {
    fn sample() -> Self {
        Self::sample_unsecurified()
    }
    fn sample_other() -> Self {
        Self::sample_securified()
    }
}

impl HasSampleValues for Persona {
    fn sample() -> Self {
        Self::sample_unsecurified()
    }
    fn sample_other() -> Self {
        Self::sample_securified()
    }
}

impl<T: Clone + Into<AddressOfAccountOrPersona> + EntityKindSpecifier + From<String>>
    AbstractEntity<T>
{
    /// mainnet
    pub(crate) fn sample_unsecurified() -> Self {
        Self::unsecurified_mainnet(0, "Alice", FactorSourceIDFromHash::fs0())
    }

    /// mainnet
    pub(crate) fn sample_securified() -> Self {
        Self::securified_mainnet(6, "Grace", |idx| {
            MatrixOfFactorInstances::m6(HierarchicalDeterministicFactorInstance::f(
                Self::entity_kind(),
                idx,
            ))
        })
    }

    fn new(name: impl AsRef<str>, security_state: impl Into<EntitySecurityState>) -> Self {
        Self {
            address: T::from(name.as_ref().to_owned()),
            security_state: security_state.into(),
        }
    }

    pub fn securified_mainnet(
        index: u32,
        name: impl AsRef<str>,
        make_matrix: fn(u32) -> MatrixOfFactorInstances,
    ) -> Self {
        Self::new(name, make_matrix(index))
    }

    pub fn unsecurified_mainnet(
        index: u32,
        name: impl AsRef<str>,
        factor_source_id: FactorSourceIDFromHash,
    ) -> Self {
        Self::new(
            name,
            EntitySecurityState::Unsecured(HierarchicalDeterministicFactorInstance::mainnet_tx(
                Self::entity_kind(),
                HDPathComponent::non_hardened(index),
                factor_source_id,
            )),
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct MatrixOfFactors<F> {
    pub threshold_factors: Vec<F>,
    pub threshold: u8,
    pub override_factors: Vec<F>,
}

impl<F> MatrixOfFactors<F>
where
    F: std::hash::Hash + std::cmp::Eq + Clone,
{
    /// # Panics
    /// Panics if threshold > threshold_factor.len()
    ///
    /// Panics if the same factor is present in both lists
    pub fn new(
        threshold_factors: impl IntoIterator<Item = F>,
        threshold: u8,
        override_factors: impl IntoIterator<Item = F>,
    ) -> Self {
        let threshold_factors = threshold_factors.into_iter().collect_vec();

        assert!(threshold_factors.len() >= threshold as usize);

        let override_factors = override_factors.into_iter().collect_vec();

        assert!(
            HashSet::<F>::from_iter(threshold_factors.clone())
                .intersection(&HashSet::<F>::from_iter(override_factors.clone()))
                .collect_vec()
                .is_empty(),
            "A factor MUST NOT be present in both threshold AND override list."
        );

        Self {
            threshold_factors,
            threshold,
            override_factors: override_factors.into_iter().collect_vec(),
        }
    }

    pub fn override_only(factors: impl IntoIterator<Item = F>) -> Self {
        Self::new([], 0, factors)
    }

    pub fn single_override(factor: F) -> Self {
        Self::override_only([factor])
    }

    pub fn threshold_only(factors: impl IntoIterator<Item = F>, threshold: u8) -> Self {
        Self::new(factors, threshold, [])
    }

    pub fn single_threshold(factor: F) -> Self {
        Self::threshold_only([factor], 1)
    }
}

pub type MatrixOfFactorInstances = MatrixOfFactors<HierarchicalDeterministicFactorInstance>;
pub type MatrixOfFactorSources = MatrixOfFactors<HDFactorSource>;

/// For unsecurified entities we map single factor -> single threshold factor.
/// Which is used by ROLA.
impl From<HierarchicalDeterministicFactorInstance> for MatrixOfFactorInstances {
    fn from(value: HierarchicalDeterministicFactorInstance) -> Self {
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

#[derive(Clone, PartialEq, Eq, std::hash::Hash, Getters, derive_more::Debug)]
#[debug("TXID({:#?})", hash.id.to_string()[..6].to_owned())]
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

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TransactionManifest {
    addresses_of_accounts_requiring_auth: Vec<AccountAddress>,
    addresses_of_personas_requiring_auth: Vec<IdentityAddress>,
}

impl TransactionManifest {
    pub fn new(
        addresses_of_accounts_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        addresses_of_personas_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> Self {
        Self {
            addresses_of_accounts_requiring_auth: addresses_of_accounts_requiring_auth
                .into_iter()
                .collect_vec(),
            addresses_of_personas_requiring_auth: addresses_of_personas_requiring_auth
                .into_iter()
                .collect_vec(),
        }
    }
    pub fn summary(&self) -> ManifestSummary {
        ManifestSummary::new(
            self.addresses_of_accounts_requiring_auth.clone(),
            self.addresses_of_personas_requiring_auth.clone(),
        )
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TransactionIntent {
    pub intent_hash: IntentHash,
    pub(crate) manifest: TransactionManifest,
}

impl TransactionIntent {
    fn with(manifest: TransactionManifest) -> Self {
        Self {
            manifest,
            intent_hash: IntentHash::generate(),
        }
    }
    pub fn new(
        addresses_of_accounts_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        addresses_of_personas_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> Self {
        Self::with(TransactionManifest::new(
            addresses_of_accounts_requiring_auth,
            addresses_of_personas_requiring_auth,
        ))
    }
    pub fn address_of<'a, 'p>(
        accounts_requiring_auth: impl IntoIterator<Item = &'a Account>,
        personas_requiring_auth: impl IntoIterator<Item = &'p Persona>,
    ) -> Self {
        Self::new(
            accounts_requiring_auth
                .into_iter()
                .map(|a| a.entity_address())
                .collect_vec(),
            personas_requiring_auth
                .into_iter()
                .map(|a| a.entity_address())
                .collect_vec(),
        )
    }
}

pub struct ManifestSummary {
    pub addresses_of_accounts_requiring_auth: Vec<AccountAddress>,
    pub addresses_of_personas_requiring_auth: Vec<IdentityAddress>,
}

impl ManifestSummary {
    pub fn new(
        addresses_of_accounts_requiring_auth: impl IntoIterator<Item = AccountAddress>,
        addresses_of_personas_requiring_auth: impl IntoIterator<Item = IdentityAddress>,
    ) -> Self {
        Self {
            addresses_of_accounts_requiring_auth: addresses_of_accounts_requiring_auth
                .into_iter()
                .collect_vec(),
            addresses_of_personas_requiring_auth: addresses_of_personas_requiring_auth
                .into_iter()
                .collect_vec(),
        }
    }
}

pub struct Profile {
    pub factor_sources: IndexSet<HDFactorSource>,
    pub accounts: HashMap<AccountAddress, Account>,
    pub personas: HashMap<IdentityAddress, Persona>,
}

impl Profile {
    pub fn new<'a, 'p>(
        factor_sources: IndexSet<HDFactorSource>,
        accounts: impl IntoIterator<Item = &'a Account>,
        personas: impl IntoIterator<Item = &'p Persona>,
    ) -> Self {
        Self {
            factor_sources,
            accounts: accounts
                .into_iter()
                .map(|a| (a.entity_address(), a.clone()))
                .collect::<HashMap<_, _>>(),
            personas: personas
                .into_iter()
                .map(|p| (p.entity_address(), p.clone()))
                .collect::<HashMap<_, _>>(),
        }
    }
    pub fn account_by_address(&self, address: AccountAddress) -> Result<Account> {
        self.accounts
            .get(&address)
            .ok_or(CommonError::UnknownAccount)
            .cloned()
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
        factor_instance: impl Into<HierarchicalDeterministicFactorInstance>,
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

#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq)]
pub enum CommonError {
    #[error("Unknown factor source")]
    UnknownFactorSource,

    #[error("Failed")]
    Failure,

    #[error("Invalid factor source kind")]
    InvalidFactorSourceKind,

    #[error("Empty FactorSources list")]
    FactorSourcesOfKindEmptyFactors,

    #[error("Unknown account")]
    UnknownAccount,

    #[error("Unknown persona")]
    UnknownPersona,
}
