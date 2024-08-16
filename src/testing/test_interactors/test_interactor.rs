use crate::prelude::*;

pub trait IsTestInteractor: Sync {
    type Request: SharedRequest;
    fn simulated_user(&self) -> SimulatedUser;
    fn do_sign(&self, request: &Self::Request) -> IndexMap<FactorSourceID, IndexSet<HDSignature>>;

    fn should_simulate_failure(&self, factor_source_ids: IndexSet<FactorSourceID>) -> bool {
        self.simulated_user()
            .simulate_failure_if_needed(factor_source_ids)
    }

    fn do_do_sign(
        &self,
        per_transaction: Vec<BatchKeySigningRequest>,
    ) -> IndexMap<FactorSourceID, IndexSet<HDSignature>> {
        per_transaction
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
            .collect::<IndexMap<FactorSourceID, IndexSet<HDSignature>>>()
    }

    fn now_sign(
        &self,
        request: &Self::Request,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>> {
        let signatures = self.do_sign(request);
        let response = BatchSigningResponse::new(signatures);
        Ok(SignWithFactorSourceOrSourcesOutcome::signed(response))
    }

    fn shared_sign(
        &self,
        request: Self::Request,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>> {
        if self.should_simulate_failure(request.factor_source_ids()) {
            return Err(CommonError::Failure);
        }
        let invalid_transactions_if_skipped = request.invalid_transactions_if_skipped();
        match self
            .simulated_user()
            .sign_or_skip(invalid_transactions_if_skipped)
        {
            SigningUserInput::Sign => self.now_sign(&request),
            SigningUserInput::Skip => Ok(SignWithFactorSourceOrSourcesOutcome::skipped(
                request.factor_source_ids(),
            )),
        }
    }
}
