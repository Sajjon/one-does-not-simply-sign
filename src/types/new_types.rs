use crate::prelude::*;

/// A signature of `intent_hash` by `entity` using `factor_source_id` and `derivation_path`, with `public_key` used for verification.
#[derive(Clone, PartialEq, Eq, Hash, derive_more::Debug)]
#[debug("HDSignature {{ instance: {:?} }}", owned_factor_instance)]
pub struct HDSignature {
    /// Hash which was signed.
    pub intent_hash: IntentHash,

    /// The ECDSA/EdDSA signature produced by the private key of the
    /// `owned_hd_factor_instance.public_key`,
    /// derived by the FactorSource identified by
    /// `owned_hd_factor_instance.factor_source_id` and which
    /// was derived at `owned_hd_factor_instance.derivation_path`.
    pub signature: Signature,

    /// The account or identity address of the entity which signed the hash,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    pub owned_factor_instance: OwnedFactorInstance,
}

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct InvalidTransactionIfSkipped {
    pub intent_hash: IntentHash,
    pub entities_which_would_fail_auth: Vec<AccountAddressOrIdentityAddress>,
}
impl InvalidTransactionIfSkipped {
    pub fn new(
        intent_hash: IntentHash,
        entities_which_would_fail_auth: Vec<AccountAddressOrIdentityAddress>,
    ) -> Self {
        Self {
            intent_hash,
            entities_which_would_fail_auth,
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

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
#[debug("{:?}: {:?}", owner, factor_instance)]
pub struct OwnedFactorInstance {
    pub factor_instance: FactorInstance,
    pub owner: AccountAddressOrIdentityAddress,
}
impl OwnedFactorInstance {
    pub fn new(factor_instance: FactorInstance, owner: AccountAddressOrIdentityAddress) -> Self {
        Self {
            factor_instance,
            owner,
        }
    }
}

impl HDSignature {
    pub fn new(
        intent_hash: IntentHash,
        signature: Signature,
        owned_factor_instance: OwnedFactorInstance,
    ) -> Self {
        Self {
            intent_hash,
            signature,
            owned_factor_instance,
        }
    }
}

#[derive(Clone, PartialEq, Eq, std::hash::Hash, derive_more::Debug)]
pub enum SignWithFactorSourceOrSourcesOutcome<T> {
    #[debug("Signed: {:?}", _0)]
    Signed(T),
    #[debug("Skipped")]
    Skipped(Vec<FactorSourceID>),
}
