use crate::prelude::*;

pub struct UseFactorSourceClient {
    driver: UseFactorSourceDriver,
}

impl UseFactorSourceClient {
    pub fn new(driver: UseFactorSourceDriver) -> Self {
        Self { driver }
    }

    pub async fn use_factor_sources(
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

                    if !coordinator.continue_if_necessary()? {
                        break;
                    }
                }
                Ok(())
            }
            UseFactorSourceDriver::SerialSingle(driver) => {
                let mut per_factor_source_outputs =
                    IndexMap::<FactorSourceID, IndexSet<HDSignature>>::new();

                'loop_0: for factor_source in factor_sources.clone() {
                    let requests_per_transaction =
                        coordinator.requests_for_serial_single_driver(&factor_source.id);

                    let mut outputs = IndexSet::<HDSignature>::new();

                    'loop_1: for (_, requests_for_transaction) in requests_per_transaction {
                        for request in requests_for_transaction {
                            let output = driver.sign(request).await?;
                            match output {
                                SignWithFactorSourceOrSourcesOutcome::Signed {
                                    produced_signatures: produced_signature,
                                } => {
                                    outputs.insert(produced_signature);
                                }
                                SignWithFactorSourceOrSourcesOutcome::Skipped {
                                    ids_of_skipped_factors_sources: _,
                                } => {
                                    outputs = IndexSet::new(); // reset outputs
                                    break 'loop_1;
                                }
                            }
                        }
                    }

                    per_factor_source_outputs.insert(factor_source.id, outputs);

                    if !coordinator.continue_if_necessary()? {
                        break 'loop_0;
                    }
                }

                let all_ids = factor_sources
                    .clone()
                    .into_iter()
                    .map(|f| f.id)
                    .collect::<IndexSet<_>>();
                let done_ids = per_factor_source_outputs
                    .iter()
                    .filter_map(|(id, signatures)| {
                        if !signatures.is_empty() {
                            Some(id)
                        } else {
                            None
                        }
                    })
                    .cloned()
                    .collect::<IndexSet<_>>();
                let ids_of_skipped_factors_sources =
                    all_ids.difference(&done_ids).cloned().collect();

                // Report the results back to the coordinator, as a batch response
                coordinator.process_batch_response(SignWithFactorSourceOrSourcesOutcome::Signed {
                    produced_signatures: BatchSigningResponse::new(per_factor_source_outputs),
                });
                coordinator.process_batch_response(SignWithFactorSourceOrSourcesOutcome::Skipped {
                    ids_of_skipped_factors_sources,
                });

                Ok(())
            }
        }
    }
}
