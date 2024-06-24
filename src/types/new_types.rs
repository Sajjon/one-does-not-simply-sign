use crate::prelude::*;

/// A factor instance which is has a known owner: AccountAddressOrIdentityAddress.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OwnedHDFactorInstance {
    /// The factor source which owns the key.
    pub factor_source_id: FactorSourceID,

    /// The address of the account or the persona this key is for.
    pub entity: AccountAddressOrIdentityAddress,

    /// The public key.
    pub public_key: PublicKey,

    /// The derivation path used to derive the `private_key` (and `public_key`).
    pub derivation_path: DerivationPath,
}

/// A signature of `intent_hash` by `entity` using `factor_source_id` and `derivation_path`, with `public_key` used for verification.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HDSignature {
    /// Hash which was signed.
    intent_hash: IntentHash,

    /// The ECDSA/EdDSA signature produced by the private key of the
    /// `owned_hd_factor_instance.public_key`,
    /// derived by the FactorSource identified by
    /// `owned_hd_factor_instance.factor_source_id` and which
    /// was derived at `owned_hd_factor_instance.derivation_path`.
    signature: Signature,

    /// The account or identity address of the entity which signed the hash,
    /// with expected public key and with derivation path to derive PrivateKey
    /// with.
    owned_hd_factor_instance: OwnedHDFactorInstance,
}
