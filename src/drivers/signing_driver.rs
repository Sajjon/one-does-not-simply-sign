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
        coordinator: &SignaturesBuildingCoordinator,
    ) {
        match self {
            // Parallel Driver: Many Factor Sources at once
            Self::ParallelBatch(driver) => {
                // Prepare the request for the driver
                let request = coordinator.request_for_parallel_batch_driver(
                    factor_sources.into_iter().map(|f| f.id).collect(),
                );

                // Produce the results from the driver
                let response = driver.sign(request).await;

                // Report the results back to the coordinator
                coordinator.process_batch_response(response);
            }
            // Serial Driver: One Factor Sources at a time
            Self::SerialBatch(driver) => {
                for factor_source in factor_sources {
                    // Prepare the request for the driver
                    let request = coordinator.request_for_serial_batch_driver(&factor_source.id);

                    // Produce the results from the driver
                    let response = driver.sign(request).await;

                    // Report the results back to the coordinator
                    coordinator.process_batch_response(response);
                }
            }
            Self::SerialSingle(driver) => {
                for factor_source in factor_sources {
                    let invalid_transactions_if_skipped =
                        coordinator.invalid_transactions_if_skipped(&factor_source.id);

                    let requests_per_transaction =
                        coordinator.inputs_for_serial_single_driver(&factor_source.id);
                    for (_, requests_for_transaction) in requests_per_transaction {
                        for partial_request in requests_for_transaction {
                            let request = SerialSingleSigningRequestFull::new(
                                partial_request,
                                invalid_transactions_if_skipped.clone(),
                            );

                            let response = driver.sign(request).await;
                            let should_continue_with_factor_source =
                                coordinator.process_single_response(response);
                            if !should_continue_with_factor_source {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
