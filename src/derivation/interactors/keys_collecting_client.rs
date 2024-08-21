use crate::prelude::*;

pub struct KeysCollectingClient {
    interactor: KeyDerivationInteractor,
}

impl KeysCollectingClient {
    pub fn new(interactor: KeyDerivationInteractor) -> Self {
        Self { interactor }
    }

    pub async fn use_factor_sources(
        &self,
        factor_sources: IndexSet<FactorSource>,
        collector: &KeysCollector,
    ) -> Result<()> {
        match &self.interactor {
            KeyDerivationInteractor::Parallel(interactor) => {
                // Prepare the request for the interactor
                let request = collector.request_for_parallel_interactor(
                    factor_sources.into_iter().map(|f| f.id).collect(),
                );
                let response = interactor.derive(request).await?;
                collector.process_batch_response(response);
            }

            KeyDerivationInteractor::Serial(interactor) => {
                for factor_source in factor_sources {
                    // Prepare the request for the interactor
                    let request = collector.request_for_serial_interactor(&factor_source.id);

                    // Produce the results from the interactor
                    let response = interactor.derive(request).await?;

                    // Report the results back to the collector
                    collector.process_batch_response(response);
                }
            }
        }
        Ok(())
    }
}
