use crate::prelude::*;

pub enum UseFactorSourceClient {
    ParallelBatch(ParallelBatchUseFactorSourcesClient),
    SerialBatch(SerialBatchUseFactorSourceClient),
    SerialSingle(SerialSingleUseFactorSourceClient),
}

impl UseFactorSourceClient {
    pub fn parallel_batch(driver: Arc<dyn ParallelBatchSigningDriver>) -> Self {
        Self::ParallelBatch(ParallelBatchUseFactorSourcesClient::new(driver))
    }

    pub fn serial_batch(driver: Arc<dyn SerialBatchSigningDriver>) -> Self {
        Self::SerialBatch(SerialBatchUseFactorSourceClient::new(driver))
    }

    pub fn serial_single(driver: Arc<dyn SerialSingleSigningDriver>) -> Self {
        Self::SerialSingle(SerialSingleUseFactorSourceClient::new(driver))
    }

    pub async fn use_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
        coordinator: &FactorResultsBuildingCoordinator,
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
