use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SerialSingleSigningRequest {
    /// The ID of the factor source used to sign each per_transaction
    pub factor_source_id: FactorSourceID,

    pub intent_hash: IntentHash,

    pub owned_factor_instance: OwnedFactorInstance,
}

/// A driver for factor source kinds which cannot sign multiple transactions
/// nor sign a single transaction with multiple keys (derivation paths).
///
/// Example of a Serial Single Signing Driver *might* be `Arculus` - we
/// do not yet know.
#[async_trait]
pub trait SerialSingleSigningDriver {
    async fn sign(&self, request: SerialSingleSigningRequest) -> HDSignature;
}

pub struct SerialSingleSigningClient {
    driver: Arc<dyn SerialSingleSigningDriver>,
}
impl SerialSingleSigningClient {
    pub fn new(driver: Arc<dyn SerialSingleSigningDriver>) -> Self {
        Self { driver }
    }
    pub async fn sign(&self, request: SerialSingleSigningRequest) -> HDSignature {
        self.driver.sign(request).await
    }
}

#[cfg(test)]
pub struct TestSerialSingleSigningDriver {
    pub simulated_user: SimulatedUser,
}
#[cfg(test)]
impl TestSerialSingleSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[cfg(test)]
#[async_trait]
impl SerialSingleSigningDriver for TestSerialSingleSigningDriver {
    async fn sign(&self, request: SerialSingleSigningRequest) -> HDSignature {
        match &self.simulated_user {
            SimulatedUser::Lazy(laziness) => match laziness {
                Laziness::AlwaysSkip => {
                    todo!()
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}
