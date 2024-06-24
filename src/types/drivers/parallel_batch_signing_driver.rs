use crate::prelude::*;



/// A collection of factor sources to use to sign, transactions with multiple keys
/// (derivations paths).
pub struct ParallelBatchSigningRequest {
    /// Per factor source, a set of transactions to sign, with
    /// multiple derivations paths.
    per_factor_source: IndexMap<FactorSourceID, IndexSet<BatchTXBatchKeySigningRequest>>,
}

/// A driver for a factor source kind which supports performing:
/// *Batch* signing *in parallel*.
///
/// Most FactorSourceKinds does in fact NOT support parallel signing,
/// i.e. signing using multiple factors sources at once, but some do,
/// typically the DeviceFactorSource does, i.e. we can load multiple
/// mnemonics from secure storage in one go and sign with all of them
/// "in parallel".
///
/// This is a bit of a misnomer, as we don't actually sign in parallel,
/// but rather we iterate through all mnemonics and sign the payload
/// with each of them in sequence
///
/// The user does not have the ability to SKIP a certain factor source,
/// instead either ALL factor sources are used to sign the transactions
/// or none.
/// 
/// Example of a Parallel Batch Signing Driver is that for DeviceFactorSource.
#[async_trait::async_trait]
pub trait ParallelBatchSigningDriver {
    async fn sign(&self, request: ParallelBatchSigningRequest) -> BatchSigningResponse;
}
