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
                println!("\n#####################\n");
                let signatures = request
                    .input
                    .per_transaction
                    .into_iter()
                    .flat_map(|r| {
                        r.signature_inputs()
                            .iter()
                            .map(|x| {
                                let sig = HDSignature::produced_signing_with_input(x.clone());
                                println!("\n✍🏻 ✍🏻 SIGNATURE ✍🏻 ✍🏻");
                                println!(
                                    "factor={:?}",
                                    &sig.input
                                        .owned_factor_instance
                                        .factor_instance()
                                        .factor_source_id
                                );
                                println!("tx={:?}", &sig.input.intent_hash);
                                println!("owner={:?}", &sig.input.owned_factor_instance.owner);
                                println!(
                                    "path={:?}",
                                    &sig.input
                                        .owned_factor_instance
                                        .factor_instance()
                                        .derivation_path()
                                );
                                sig
                            })
                            .collect::<IndexSet<_>>()
                    })
                    .collect::<IndexSet<HDSignature>>();
                println!("\nSIGNATURES END\n\n#####################\n");
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
            SigningUserInput::Skip => {
                Ok(SignWithFactorSourceOrSourcesOutcome::skipped_factor_source(
                    request.input.factor_source_id,
                ))
            }
        }
    }
}
