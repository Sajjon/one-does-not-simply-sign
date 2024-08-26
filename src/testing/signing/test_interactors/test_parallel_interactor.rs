use crate::prelude::*;

pub struct TestSigningParallelInteractor {
    simulated_user: SimulatedUser,
}

impl TestSigningParallelInteractor {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[async_trait::async_trait]
impl IsTestInteractor for TestSigningParallelInteractor {
    fn simulated_user(&self) -> SimulatedUser {
        self.simulated_user.clone()
    }
}

#[async_trait::async_trait]
impl SignWithFactorParallelInteractor for TestSigningParallelInteractor {
    async fn sign(
        &self,
        request: ParallelBatchSigningRequest,
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
                    .flat_map(|(_, v)| {
                        v.per_transaction
                            .iter()
                            .flat_map(|x| {
                                x.signature_inputs()
                                    .iter()
                                    .map(|y| HDSignature::produced_signing_with_input(y.clone()))
                                    .collect_vec()
                            })
                            .collect::<IndexSet<HDSignature>>()
                    })
                    .collect::<IndexSet<HDSignature>>();

                let signatures = signatures
                    .into_iter()
                    .into_group_map_by(|x| x.factor_source_id());
                let response = BatchSigningResponse::new(
                    signatures
                        .into_iter()
                        .map(|(k, v)| (k, IndexSet::from_iter(v)))
                        .collect(),
                );

                Ok(SignWithFactorSourceOrSourcesOutcome::signed(response))
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
