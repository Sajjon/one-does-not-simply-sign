use crate::prelude::*;

pub struct TestParallelBatchSigningDriver {
    simulated_user: SimulatedUser,
}

impl TestParallelBatchSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[async_trait]
impl IsTestUseFactorSourcesDriver for TestParallelBatchSigningDriver {
    fn simulated_user(&self) -> SimulatedUser {
        self.simulated_user.clone()
    }
}

#[async_trait]
impl ParallelBatchUseFactorSourcesDriver for TestParallelBatchSigningDriver {
    async fn sign(
        &self,
        request: SignWithFactorParallelInteractor,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>> {
        if self.should_simulate_failure(request.per_factor_source.keys().cloned().collect()) {
            return Err(CommonError::Failure);
        }
        match self
            .simulated_user
            .sign_or_skip(request.invalid_transactions_if_skipped)
        {
            SigningUserInput::Sign => {
                let signatures = request
                    .per_factor_source
                    .iter()
                    .map(|(k, v)| {
                        let value = v
                            .per_transaction
                            .iter()
                            .flat_map(|x| {
                                x.signature_inputs()
                                    .iter()
                                    .map(|y| HDSignature::produced_signing_with_input(y.clone()))
                                    .collect_vec()
                            })
                            .collect::<IndexSet<HDSignature>>();
                        (*k, value)
                    })
                    .collect::<IndexMap<FactorSourceID, IndexSet<HDSignature>>>();

                let response = SignWithFactorSourceOrSourcesOutcome::signed(
                    BatchSigningResponse::new(signatures),
                );
                Ok(response)
            }

            SigningUserInput::Skip => Ok(SignWithFactorSourceOrSourcesOutcome::skipped(
                request
                    .per_factor_source
                    .keys()
                    .cloned()
                    .collect::<IndexSet<_>>(),
            )),
        }
    }
}
