use crate::prelude::*;

pub struct TestSigningParallelInteractor {
    simulated_user: SimulatedUser,
}

impl TestSigningParallelInteractor {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

impl IsTestInteractor for TestSigningParallelInteractor {
    fn simulated_user(&self) -> SimulatedUser {
        self.simulated_user.clone()
    }
}

fn do_sign(
    per_factor_source: IndexMap<FactorSourceID, BatchTXBatchKeySigningRequest>,
) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>> {
    let signatures = per_factor_source
        .into_iter()
        .flat_map(|(_, v)| do_do_sign(v.per_transaction))
        .collect();
    let response = BatchSigningResponse::new(signatures);
    Ok(SignWithFactorSourceOrSourcesOutcome::signed(response))
}

#[async_trait]
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
            .sign_or_skip(request.invalid_transactions_if_skipped())
        {
            SigningUserInput::Sign => do_sign(request.per_factor_source),
            SigningUserInput::Skip => Ok(SignWithFactorSourceOrSourcesOutcome::skipped(
                request.factor_source_ids(),
            )),
        }
    }
}
