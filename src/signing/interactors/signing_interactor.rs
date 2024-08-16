use crate::prelude::*;

/// A collection of "interactors" which can sign transactions.
pub trait SignatureCollectingInteractors {
    fn interactor_for(&self, kind: FactorSourceKind) -> SigningInteractor;
}

/// A collection of factor sources to use to sign, transactions with multiple keys
/// (derivations paths).
pub struct ParallelBatchSigningRequest {
    /// Per factor source, a set of transactions to sign, with
    /// multiple derivations paths.
    pub per_factor_source: IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>,

    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source.
    invalid_transactions_if_skipped: IndexSet<InvalidTransactionIfSkipped>,
}

impl ParallelBatchSigningRequest {
    pub fn new(
        per_factor_source: IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>,
        invalid_transactions_if_skipped: IndexSet<InvalidTransactionIfSkipped>,
    ) -> Self {
        Self {
            per_factor_source,
            invalid_transactions_if_skipped,
        }
    }
    pub fn invalid_transactions_if_skipped(&self) -> IndexSet<InvalidTransactionIfSkipped> {
        self.invalid_transactions_if_skipped.clone()
    }
    pub fn factor_source_ids(&self) -> IndexSet<FactorSourceID> {
        self.per_factor_source
            .keys()
            .cloned()
            .collect::<IndexSet<_>>()
    }
}

/// A batch signing request used with a SignWithFactorSerialInteractor, containing
/// a collection of transactions to sign with multiple keys (derivation paths),
/// and a collection of transactions which would be invalid if the user skips
/// signing with this factor source.
pub struct SerialBatchSigningRequest {
    pub input: BatchTXBatchKeySigningRequest,
    /// A collection of transactions which would be invalid if the user skips
    /// signing with this factor source.
    pub invalid_transactions_if_skipped: Vec<InvalidTransactionIfSkipped>,
}

impl SerialBatchSigningRequest {
    pub fn new(
        input: BatchTXBatchKeySigningRequest,
        invalid_transactions_if_skipped: Vec<InvalidTransactionIfSkipped>,
    ) -> Self {
        Self {
            input,
            invalid_transactions_if_skipped,
        }
    }
}

/// The response of a batch signing request, either a Parallel or Serial signing
/// request, matters not, because the goal is to have signed all transactions with
/// enough keys (derivation paths) needed for it to be valid when submitted to the
/// Radix network.
#[derive(Clone, PartialEq, Eq, derive_more::Debug)]
#[debug("BatchSigningResponse {{ signatures: {:?} }}", signatures.values().map(|f| format!("{:?}", f)).join(", "))]
pub struct BatchSigningResponse {
    pub signatures: IndexMap<FactorSourceID, IndexSet<HDSignature>>,
}

impl BatchSigningResponse {
    pub fn new(signatures: IndexMap<FactorSourceID, IndexSet<HDSignature>>) -> Self {
        Self { signatures }
    }
}

/// A interactor for a factor source kind which supports *Batch* usage of
/// multiple factor sources in parallel.
///
/// Most FactorSourceKinds does in fact NOT support parallel usage,
/// e.g. signing using multiple factors sources at once, but some do,
/// typically the DeviceFactorSource does, i.e. we can load multiple
/// mnemonics from secure storage in one go and sign with all of them
/// "in parallel".
///
/// This is a bit of a misnomer, as we don't actually use them in parallel,
/// but rather we iterate through all mnemonics and derive public keys/
/// or sign a payload with each of them in sequence
///
/// The user does not have the ability to SKIP a certain factor source,
/// instead either ALL factor sources are used to sign the transactions
/// or none.
///
/// Example of a Parallel Batch Signing Driver is that for DeviceFactorSource.

#[async_trait::async_trait]
pub trait SignWithFactorParallelInteractor {
    async fn sign(
        &self,
        request: ParallelBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>>;
}

/// A interactor for a factor source kind which support performing
/// *Batch* signing *serially*.
///
/// Meaning we initiate and prompt user for signing with one factor source
/// at a time, where each signing operation is support batch signing, that is
/// signing multiple transactions each with multiple keys (derivations paths).
///
/// The user might chose to SKIP the current factor source, and move on to the
/// next one.
///
/// Example of a Serial Batch Signing Driver is SecurityQuestionsFactorSource,
/// where it does not make any sense to let user in parallel answer multiple
/// questions from different security questions factor sources (in fact we
/// might not even even allow multiple SecurityQuestionsFactorSources to be used).
#[async_trait]
pub trait SignWithFactorSerialInteractor {
    async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>>;
}

/// An interactor which can sign transactions - either in parallel or serially.
pub enum SigningInteractor {
    Parallel(Arc<dyn SignWithFactorParallelInteractor>),
    Serial(Arc<dyn SignWithFactorSerialInteractor>),
}

impl SigningInteractor {
    pub fn parallel(interactor: Arc<dyn SignWithFactorParallelInteractor>) -> Self {
        Self::Parallel(interactor)
    }

    pub fn serial(interactor: Arc<dyn SignWithFactorSerialInteractor>) -> Self {
        Self::Serial(interactor)
    }
}
