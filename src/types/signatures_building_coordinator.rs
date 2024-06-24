use crate::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MaybeSignedTransactions {
    /// Collection of transactions which might be signed or not.
    transactions: IndexMap<IntentHash, IndexSet<HDSignature>>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SignaturesOutcome {
    successful_transactions: MaybeSignedTransactions,
    failed_transactions: MaybeSignedTransactions,
}

pub enum SigningDriver {
    ParallelBatch(ParallelBatchSigningClient),
    SerialBatch(SerialBatchSigningClient),
    SerialSingle(SerialSingleSigningClient),
}

impl SigningDriver {
    pub fn parallel_batch(driver: Arc<dyn ParallelBatchSigningDriver>) -> Self {
        Self::ParallelBatch(ParallelBatchSigningClient::new(driver))
    }
    pub fn serial_batch(driver: Arc<dyn SerialBatchSigningDriver>) -> Self {
        Self::SerialBatch(SerialBatchSigningClient::new(driver))
    }
    pub fn serial_single(driver: Arc<dyn SerialSingleSigningDriver>) -> Self {
        Self::SerialSingle(SerialSingleSigningClient::new(driver))
    }
    pub async fn sign(
        &self,
        kind: FactorSourceKind,
        factor_sources: IndexSet<FactorSource>,
        signatures_building_coordinator: &SignaturesBuildingCoordinator,
    ) {
        match self {
            Self::ParallelBatch(driver) => todo!(),
            Self::SerialBatch(driver) => todo!(),
            Self::SerialSingle(driver) => todo!(),
        }
    }
}

pub trait IsSigningDriversContext {
    fn driver_for_factor_source_kind(&self, kind: FactorSourceKind) -> SigningDriver;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigningUserInput {
    Sign,
    Skip,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimulatedUser {
    /// Emulation of a "prudent" user, that signs with all factors sources, i.e.
    /// she never ever "skips" a factor source
    Prudent,

    /// Emulation of a "lazy" user, that skips signing with as many factor
    /// sources as possible.
    Lazy(Laziness),

    /// Emulation of a "random" user, that skips signing some factor sources
    ///  at random.
    Random,
}

#[cfg(test)]
impl SimulatedUser {
    pub fn lazy_always_skip() -> Self {
        Self::Lazy(Laziness::AlwaysSkip)
    }
    /// Skips only if `invalid_tx_if_skipped` is empty
    pub fn lazy_sign_minimum() -> Self {
        Self::Lazy(Laziness::SignMinimum)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Laziness {
    SignMinimum,
    AlwaysSkip,
}

#[cfg(test)]
impl SimulatedUser {
    async fn sign_or_skip(
        &self,
        factor_source: &FactorSource,
        invalid_tx_if_skipped: IndexSet<InvalidTransactionIfSkipped>,
    ) -> SigningUserInput {
        use rand::prelude::*;
        match self {
            SimulatedUser::Prudent => SigningUserInput::Sign,
            SimulatedUser::Lazy(laziness) => match laziness {
                Laziness::AlwaysSkip => SigningUserInput::Skip,
                Laziness::SignMinimum => {
                    if invalid_tx_if_skipped.is_empty() {
                        SigningUserInput::Skip
                    } else {
                        SigningUserInput::Sign
                    }
                }
            },
            SimulatedUser::Random => {
                let mut rng = rand::thread_rng();
                let num: f64 = rng.gen(); // generates a float between 0 and 1
                if num > 0.5 {
                    SigningUserInput::Skip
                } else {
                    SigningUserInput::Sign
                }
            }
        }
    }
}

#[cfg(test)]
unsafe impl Sync for SimulatedUser {}

#[cfg(test)]

pub struct TestSigningDriversContext {
    pub simulated_user: SimulatedUser,
}

#[cfg(test)]
impl IsSigningDriversContext for TestSigningDriversContext {
    fn driver_for_factor_source_kind(&self, kind: FactorSourceKind) -> SigningDriver {
        match kind {
            FactorSourceKind::Device => SigningDriver::parallel_batch(Arc::new(
                TestParallelBatchSigningDriver::new(self.simulated_user.clone()),
            )),
            FactorSourceKind::Arculus => SigningDriver::serial_single(Arc::new(
                TestSerialSingleSigningDriver::new(self.simulated_user.clone()),
            )),
            _ => SigningDriver::serial_batch(Arc::new(TestSerialBatchSigningDriver::new(
                self.simulated_user.clone(),
            ))),
        }
    }
}

/// A coordinator which gathers signatures from several factor sources of different
/// kinds for many transactions and for potentially multiple derivation paths
/// for each transaction.
pub struct SignaturesBuildingCoordinator {
    signing_drivers_context: Arc<dyn IsSigningDriversContext>,
    /// Factor sources grouped by kind, sorted according to "signing order",
    /// that is, we want to control which factor source kind users signs with
    /// first, second etc, e.g. typically we prompt user to sign with Ledgers
    /// first, and if a user might lack access to that Ledger device, then it is
    /// best to "fail fast", otherwise we might waste the users time, if she has
    /// e.g. answered security questions and then is asked to sign with a Ledger
    /// she might not have handy at the moment - or might not be in front of a
    /// computer and thus unable to make a connection between the Radix Wallet
    /// and a Ledger device.
    factors_of_kind: IndexMap<FactorSourceKind, IndexSet<FactorSource>>,
}

impl SignaturesBuildingCoordinator {}
