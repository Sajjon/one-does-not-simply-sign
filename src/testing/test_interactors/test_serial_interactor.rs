use crate::prelude::*;

pub struct TestSigningSerialInteractor {
    simulated_user: SimulatedUser,
}

impl TestSigningSerialInteractor {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

impl IsTestInteractor for TestSigningSerialInteractor {
    fn simulated_user(&self) -> SimulatedUser {
        self.simulated_user.clone()
    }
}

#[async_trait]
impl SignWithFactorSerialInteractor for TestSigningSerialInteractor {
    async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>> {
        if self.should_simulate_failure(IndexSet::from_iter([request.input.factor_source_id])) {
            return Err(CommonError::Failure);
        }
        let invalid_transactions_if_skipped = request.invalid_transactions_if_skipped;
        match self
            .simulated_user
            .sign_or_skip(invalid_transactions_if_skipped)
        {
            SigningUserInput::Sign => do_sign(
                request
                    .input
                    .per_transaction
                    .into_iter()
                    .map(|x| {
                        (
                            x.factor_source_id,
                            BatchTXBatchKeySigningRequest::new(
                                x.factor_source_id,
                                x.signature_inputs()
                                    .iter()
                                    .map(|y| {
                                        BatchKeySigningRequest::new(
                                            y.intent_hash.clone(),
                                            x.factor_source_id,
                                            IndexSet::from_iter([y.owned_factor_instance.clone()]),
                                        )
                                    })
                                    .collect::<IndexSet<BatchKeySigningRequest>>(),
                            ),
                        )
                    })
                    .collect::<IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>>(),
            ),
            SigningUserInput::Skip => {
                Ok(SignWithFactorSourceOrSourcesOutcome::skipped_factor_source(
                    request.input.factor_source_id,
                ))
            }
        }
    }
}
