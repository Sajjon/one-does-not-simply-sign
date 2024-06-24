use crate::prelude::*;

/// A factor instance which is has a known owner: AccountAddressOrIdentityAddress.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OwnedHDFactorInstance {
    /// The factor source which owns the key.
    pub factor_source_id: FactorSourceID,

    /// The address of the account or the persona this key is for.
    pub owner: AccountAddressOrIdentityAddress,

    /// The public key.
    pub factor_instance: FactorInstance,
}

/// A signature of `intent_hash` by `entity` using `factor_source_id` and `derivation_path`, with `public_key` used for verification.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
    pub owned_factor_instance: OwnedHDFactorInstance,
}

#[derive(Debug, Clone, PartialEq, Eq, std::hash::Hash)]
pub enum SignWithFactorSourceOrSourcesOutcome {
    Signed(Vec<HDSignature>), // want IndexSet
    Skipped,
}
