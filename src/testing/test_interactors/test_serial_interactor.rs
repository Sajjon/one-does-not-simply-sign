use crate::prelude::*;

pub struct TestSigningSerialInteractor {
    simulated_user: SimulatedUser,
}

impl TestSigningSerialInteractor {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[async_trait]
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
            SigningUserInput::Sign => {
                let signatures = request
                    .input
                    .per_transaction
                    .into_iter()
                    .map(|r| {
                        let key = r.factor_source_id;

                        let value = r
                            .signature_inputs()
                            .iter()
                            .map(|x| HDSignature::produced_signing_with_input(x.clone()))
                            .collect::<IndexSet<_>>();
                        (key, value)
                    })
                    .collect::<IndexMap<FactorSourceID, IndexSet<HDSignature>>>();
                let response = BatchSigningResponse::new(signatures);
                Ok(SignWithFactorSourceOrSourcesOutcome::signed(response))
            }
            SigningUserInput::Skip => {
                Ok(SignWithFactorSourceOrSourcesOutcome::skipped_factor_source(
                    request.input.factor_source_id,
                ))
            }
        }
    }
}