use crate::prelude::*;

pub struct SignWithFactorClient {
    interactor: InteractorParallelOrSerial,
}

impl SignWithFactorClient {
    pub fn new(interactor: InteractorParallelOrSerial) -> Self {
        Self { interactor }
    }

    pub async fn use_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
        collector: &SignaturesCollector,
    ) -> Result<()> {
        match &self.interactor {
            // Parallel Interactor: Many Factor Sources at once
            InteractorParallelOrSerial::Parallel(interactor) => {
                // Prepare the request for the interactor
                let request = collector.request_for_parallel_interactor(
                    factor_sources.into_iter().map(|f| f.id).collect(),
                );
                let response = interactor.sign(request).await?;
                collector.process_batch_response(response);
            }

            // Serial Interactor: One Factor Sources at a time
            // After each factor source we pass the result to the collector
            // updating its internal state so that we state about being able
            // to skip the next factor source or not.
            InteractorParallelOrSerial::Serial(interactor) => {
                for factor_source in factor_sources {
                    // Prepare the request for the interactor
                    let request = collector.request_for_serial_interactor(&factor_source.id);

                    // Produce the results from the interactor
                    let response = interactor.sign(request).await?;

                    // Report the results back to the collector
                    collector.process_batch_response(response);

                    if !collector.continue_if_necessary()? {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
