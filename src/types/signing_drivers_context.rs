use crate::prelude::*;

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
        factor_sources: IndexSet<FactorSource>,
        signatures_building_coordinator: &SignaturesBuildingCoordinator,
    ) {
        match self {
            Self::ParallelBatch(driver) => {
                let per_factor_source = factor_sources
                    .clone()
                    .into_iter()
                    .map(|f| {
                        let key = f.id.clone();
                        let value = signatures_building_coordinator
                            .input_for_parallel_batch_driver(f.clone());
                        (key, value)
                    })
                    .collect::<IndexMap<FactorSourceID, IndexSet<BatchTXBatchKeySigningRequest>>>();
                let request = ParallelBatchSigningRequest::new(per_factor_source);
                let response = driver.sign(request).await;
                signatures_building_coordinator.process_batch_response(response, factor_sources);
            }
            Self::SerialBatch(driver) => todo!(),
            Self::SerialSingle(driver) => todo!(),
        }
    }
}

pub trait IsSigningDriversContext {
    fn driver_for_factor_source_kind(&self, kind: FactorSourceKind) -> SigningDriver;
}

#[cfg(test)]

pub struct TestSigningDriversContext {
    pub simulated_user: SimulatedUser,
}
#[cfg(test)]
impl TestSigningDriversContext {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
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
