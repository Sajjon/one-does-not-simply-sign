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
        let factor_source_count = factor_sources.len();
        match self {
            Self::ParallelBatch(driver) => {
                println!("🚀 Signing with Parallel Batch driver...");
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
                println!("🚗 Signing with Serial Batch driver... #{} factor sources, signing with one factor source at a time", factor_source_count);
                for factor_source in factor_sources {
                    println!(
                        "🚗 Signing with Serial Batch driver, signing with factor source: {:?}",
                        factor_source.id
                    );
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

                    println!(
                        "☑️ Got 1 response (of #{}) from Serial Batch driver: {:?}",
                        factor_source_count, &response
                    );
                    signatures_building_coordinator.process_batch_response(response);
                }
            }
            Self::SerialSingle(driver) => {
                println!("🐌 Signing with Serial Single driver...#{} factor sources, signing with one factor source at a time, many times, one time for each factor instance", factor_source_count);
                for factor_source in factor_sources {
                    let invalid_transactions_if_skipped = signatures_building_coordinator
                        .invalid_transactions_if_skipped(&factor_source.id);

                    println!(
                        "🐌 Signing with Serial Single, signing with factor source: {:?}",
                        factor_source.id
                    );
                    let requests_per_transaction = signatures_building_coordinator
                        .inputs_for_serial_single_driver(&factor_source.id);
                    for (_, requests_for_transaction) in requests_per_transaction {
                        for partial_request in requests_for_transaction {
                            let request = SerialSingleSigningRequestFull::new(
                                partial_request,
                                invalid_transactions_if_skipped.clone(),
                            );
                            println!(
                                "🐌 Signing with Serial Single, signing with instance: {:?}",
                                &request.input.owned_factor_instance
                            );
                            let response = driver.sign(request).await;
                            let should_continue_with_factor_source =
                                signatures_building_coordinator.process_single_response(response);
                            if !should_continue_with_factor_source {
                                println!("⁉️ Breaking, should continue with next factor source....");
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
