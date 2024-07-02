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

    pub async fn did_fail_ask_if_retry(&self, factor_source_ids: IndexSet<FactorSourceID>) -> bool {
        match self {
            UseFactorSourceDriver::ParallelBatch(driver) => {
                driver.did_fail_ask_if_retry(factor_source_ids).await
            }
            UseFactorSourceDriver::SerialBatch(driver) => {
                driver.did_fail_ask_if_retry(factor_source_ids).await
            }
            UseFactorSourceDriver::SerialSingle(driver) => {
                driver.did_fail_ask_if_retry(factor_source_ids).await
            }
        }
    }
}

pub struct UseFactorSourceClient {
    driver: UseFactorSourceDriver,
}

impl UseFactorSourceClient {
    pub fn new(driver: UseFactorSourceDriver) -> Self {
        Self { driver }
    }

    /// `Ok(false)` means that we failed to sign and failed to retry.
    pub async fn use_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
        coordinator: &FactorResultsBuildingCoordinator,
    ) -> Result<bool> {
        match self
            .do_use_factor_sources(factor_sources.clone(), coordinator)
            .await
        {
            Ok(()) => Ok(true),
            Err(_) => {
                // Ask user if she wants to retry.
                if self
                    .driver
                    .did_fail_ask_if_retry(
                        factor_sources.clone().into_iter().map(|f| f.id).collect(),
                    )
                    .await
                {
                    // recursive call (Box::pin is needed since we are recursively calling ourselves and we are async)
                    Box::pin(self.use_factor_sources(factor_sources.clone(), coordinator)).await
                } else {
                    coordinator.process_batch_response(
                        SignWithFactorSourceOrSourcesOutcome::Skipped {
                            ids_of_skipped_factors_sources: factor_sources
                                .into_iter()
                                .map(|f| f.id)
                                .collect(),
                        },
                    );
                    Ok(false)
                }
            }
        }
    }

    async fn do_use_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
        coordinator: &FactorResultsBuildingCoordinator,
    ) -> Result<()> {
        match &self.driver {
            // Parallel Driver: Many Factor Sources at once
            UseFactorSourceDriver::ParallelBatch(driver) => {
                // Prepare the request for the driver
                let request = coordinator.request_for_parallel_batch_driver(
                    factor_sources.into_iter().map(|f| f.id).collect(),
                );
                let response = driver.sign(request).await?;
                coordinator.process_batch_response(response);

                Ok(())
            }

            // Serial Driver: One Factor Sources at a time
            // After each factor source we pass the result to the coordinator
            // updating its internal state so that we state about being able
            // to skip the next factor source or not.
            UseFactorSourceDriver::SerialBatch(driver) => {
                for factor_source in factor_sources {
                    // Prepare the request for the driver
                    let request = coordinator.request_for_serial_batch_driver(&factor_source.id);

                    // Produce the results from the driver
                    let response = driver.sign(request).await?;

                    // Report the results back to the coordinator
                    coordinator.process_batch_response(response);
                }
                Ok(())
            }
            UseFactorSourceDriver::SerialSingle(driver) => {
                for factor_source in factor_sources {
                    let requests_per_transaction =
                        coordinator.requests_for_serial_single_driver(&factor_source.id);
                    let mut outputs = IndexSet::<HDSignature>::new();
                    for (_, requests_for_transaction) in requests_per_transaction {
                        for request in requests_for_transaction {
                            let output = driver.sign(request).await?;
                            match output {
                                SignWithFactorSourceOrSourcesOutcome::Signed {
                                    produced_signatures: produced_signature,
                                } => {
                                    outputs.insert(produced_signature);
                                }
                                SignWithFactorSourceOrSourcesOutcome::Skipped {
                                    ids_of_skipped_factors_sources,
                                } => {
                                    coordinator.process_batch_response(
                                        SignWithFactorSourceOrSourcesOutcome::Skipped {
                                            ids_of_skipped_factors_sources,
                                        },
                                    );
                                    return Ok(());
                                }
                            }
                        }
                    }
                    let batch_response = BatchSigningResponse::new(IndexMap::from_iter([(
                        factor_source.id,
                        outputs,
                    )]));

                    // Report the results back to the coordinator, as a batch response
                    coordinator.process_batch_response(
                        SignWithFactorSourceOrSourcesOutcome::Signed {
                            produced_signatures: batch_response,
                        },
                    );
                }
                Ok(())
            }
        }
    }
}
