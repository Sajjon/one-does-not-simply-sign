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
                        let key = f.id;
                        let value =
                            signatures_building_coordinator.input_for_parallel_batch_driver(&f.id);
                        (key, value)
                    })
                    .collect::<IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>>();
                let invalid_transactions_if_skipped = signatures_building_coordinator
                    .invalid_transactions_if_skipped_factor_sources(
                        factor_sources.iter().map(|f| f.id).collect::<IndexSet<_>>(),
                    );
                let request = ParallelBatchSigningRequest::new(
                    per_factor_source,
                    invalid_transactions_if_skipped,
                );
                let response = driver.sign(request).await;
                signatures_building_coordinator.process_batch_response(response);
            }
            Self::SerialBatch(driver) => {
                for factor_source in factor_sources {
                    let batch_signing_request = signatures_building_coordinator
                        .input_for_parallel_batch_driver(&factor_source.id);

                    let request = SerialBatchSigningRequest::new(
                        batch_signing_request,
                        signatures_building_coordinator
                            .invalid_transactions_if_skipped(&factor_source.id)
                            .into_iter()
                            .collect_vec(),
                    );

                    let response = driver.sign(request).await;
                    signatures_building_coordinator.process_batch_response(response);
                }
            }
            Self::SerialSingle(driver) => {
                for factor_source in factor_sources {
                    let invalid_transactions_if_skipped = signatures_building_coordinator
                        .invalid_transactions_if_skipped(&factor_source.id);

                    let requests_per_transaction = signatures_building_coordinator
                        .inputs_for_serial_single_driver(&factor_source.id);
                    for (_, requests_for_transaction) in requests_per_transaction {
                        for partial_request in requests_for_transaction {
                            let request = SerialSingleSigningRequestFull::new(
                                partial_request,
                                invalid_transactions_if_skipped.clone(),
                            );

                            let response = driver.sign(request).await;
                            let should_continue_with_factor_source =
                                signatures_building_coordinator.process_single_response(response);
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
