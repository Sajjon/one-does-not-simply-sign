use crate::prelude::*;

pub enum UseFactorSourceDriver {
    ParallelBatch(Arc<dyn ParallelBatchUseFactorSourcesDriver>),
    SerialBatch(Arc<dyn SerialBatchUseFactorSourceDriver>),
    SerialSingle(Arc<dyn SerialSingleUseFactorSourceDriver>),
}

impl UseFactorSourceDriver {
    pub fn parallel_batch(driver: Arc<dyn ParallelBatchUseFactorSourcesDriver>) -> Self {
        Self::ParallelBatch(driver)
    }

    pub fn serial_batch(driver: Arc<dyn SerialBatchUseFactorSourceDriver>) -> Self {
        Self::SerialBatch(driver)
    }

    pub fn serial_single(driver: Arc<dyn SerialSingleUseFactorSourceDriver>) -> Self {
        Self::SerialSingle(driver)
    }
}

pub struct UseFactorSourceClient;

impl UseFactorSourceClient {
    pub async fn use_factor_sources(
        &self,
        driver: UseFactorSourceDriver,
        factor_sources: IndexSet<FactorSource>,
        coordinator: &FactorResultsBuildingCoordinator,
    ) {
        match driver {
            // Parallel Driver: Many Factor Sources at once
            UseFactorSourceDriver::ParallelBatch(driver) => {
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
            UseFactorSourceDriver::SerialBatch(driver) => {
                for factor_source in factor_sources {
                    // Prepare the request for the driver
                    let request = coordinator.request_for_serial_batch_driver(&factor_source.id);

                    // Produce the results from the driver
                    let response = driver.sign(request).await;

                    // Report the results back to the coordinator
                    coordinator.process_batch_response(response);
                }
            }
            UseFactorSourceDriver::SerialSingle(driver) => {
                for factor_source in factor_sources {
                    let requests_per_transaction =
                        coordinator.requests_for_serial_single_driver(&factor_source.id);
                    for (_, requests_for_transaction) in requests_per_transaction {
                        for request in requests_for_transaction {
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
