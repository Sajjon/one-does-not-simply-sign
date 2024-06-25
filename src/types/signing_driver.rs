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
                    .collect::<IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>>();
                let request = ParallelBatchSigningRequest::new(per_factor_source);
                let response = driver.sign(request).await;
                signatures_building_coordinator.process_batch_response(response, factor_sources);
            }
            Self::SerialBatch(driver) => todo!(),
            Self::SerialSingle(driver) => todo!(),
        }
    }
}
