//! Question: Is there any difference between BatchSigningDrivers and 
//! SingleSigningDrivers other than the fact that BatchSigningDerivers can sign
//! many transactions with many derivations paths at once? 

mod types;

/// A factor instance which is has a known owner: AccountAddressOrIdentityAddress.
pub struct OwnedHDFactorInstance {
    /// The factor source which owns the key.
    factor_source_id: FactorSourceID,

    /// The address of the account or the persona this key is for.
    entity: AccountAddressOrIdentityAddress,

    /// The public key.
    public_key: PublicKey,

    /// The derivation path used to derive the `private_key` (and `public_key`).
    derivation_path: DerivationPath,
}


/// A signature of `intent_hash` by `entity` using `factor_source_id` and `derivation_path`, with `public_key` used for verification.
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


/// A batch of keys (derivation paths) all being factor instances of a FactorSource
/// with id `factor_source_id` to sign a single transaction with, which hash
/// is `intent_hash`.
pub struct BatchKeySigningRequest {
    /// Hash to sign
    intent_hash: IntentHash,

    /// ID of factor to use to sign
    factor_source_id: FactorSourceID,

    /// The derivation paths to use to derive the private keys to sign with. The
    /// `factor_source_id` of each item must match `factor_source_id`.
    owned_factor_instances: IndexSet<OwnedHDFactorInstance>,
}

/// A batch of transactions each batching over multiple keys (derivation paths)
/// to sign each transaction with.
pub struct BatchTXBatchKeySigningRequest {

    /// The ID of the factor source used to sign each per_transaction
    factor_source_id: FactorSourceID,

    // The `factor_source_id` of each item must match `self.factor_source_id`.
    per_transaction: IndexSet<IntentHash, IndexSet<BatchKeySigningRequest>>,
}

/// A collection of factor sources to use to sign, transactions with multiple keys
/// (derivations paths).
pub struct ParallelBatchSigningDriverRequest {
    /// Per factor source, a set of transactions to sign, with
    /// multiple derivations paths.
    per_factor_source: IndexMap<FactorSourceID, IndexSet<BatchTXBatchKeySigningRequest>>,
}


pub struct ParallelBatchSigningDriverResponse {
    /// The `IntentHash` should match the `intent_hash` of each HDSignature.
    signatures: HashMap<IntentHash, IndexSet<HDSignature>>,
}


#[async_trait::async_trait]
pub trait ParallelBatchSigningDriver {
    async fn sign(&self, request: ParallelBatchSigningDriverRequest) -> ParallelBatchSigningDriverResponse;
}

struct ParallelBatchSigningClient {
    driver: Arc<dyn ParallelBatchSigningDriver>,
}

impl ParallelBatchSigningClient {
    async fn sign(&self, request: ParallelBatchSigningRequest) -> ParallelBatchSigningResponse {
        self.driver.sign(request).await
    }
}



/// A coordinator which gathers signatures from several factor sources of different
/// kinds for many transactions and for potentially multiple derivation paths
/// for each transaction.
pub struct SignaturesBuildingCoordinator;

/// Typically this would be one driver per factor source kind, but
/// we make some assumptions here about us having a shared driver
/// for all kinds.
///
/// Most FactorSourceKinds does in fact NOT support parallel signing,
/// i.e. signing using multiple factors sources at once, but some do,
/// typically the DeviceFactorSource does, i.e. we can load multiple
/// mnemonics from secure storage in one go and sign with all of them
/// "in parallel".
///
/// This is a bit of a misnomer, as we don't actually sign in parallel,
/// but rather we iterate through all mnemonics and sign the 2D-batch
/// payload with each of them in sequence. By 2D batch payload we mean
/// to sign multiple transactions each with multiple derivation paths.
pub struct ParallelSigningDriver;

pub mod prelude {
    pub use crate::types::*;

    pub use indexmap::*;
    pub use indexset::*;
}

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

pub use prelude::*;

mod tests {
    #[test]
    fn test() {}
}
