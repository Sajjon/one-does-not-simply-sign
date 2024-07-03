use crate::prelude::*;

pub struct TestSerialSingleSigningDriver {
    simulated_user: SimulatedUser,
}

impl TestSerialSingleSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[async_trait]
impl IsTestUseFactorSourcesDriver for TestSerialSingleSigningDriver {
    fn simulated_user(&self) -> SimulatedUser {
        self.simulated_user.clone()
    }
}

#[async_trait]
impl SerialSingleUseFactorSourceDriver for TestSerialSingleSigningDriver {
    async fn sign(
        &self,
        request: SerialSingleSigningRequestFull,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<HDSignature>> {
        if self.should_simulate_failure(IndexSet::from_iter([request.input.factor_source_id])) {
            return Err(CommonError::Failure);
        }
        let response = match self
            .simulated_user
            .sign_or_skip(request.invalid_transactions_if_skipped)
        {
            SigningUserInput::Sign => SignWithFactorSourceOrSourcesOutcome::signed(
                HDSignature::produced_signing_with_input(request.input.signature_input()),
            ),
            SigningUserInput::Skip => SignWithFactorSourceOrSourcesOutcome::skipped_factor_source(
                request.input.factor_source_id,
            ),
        };
        Ok(response)
    }
}
