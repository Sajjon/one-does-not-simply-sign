use crate::prelude::*;

pub struct TestSigningDriversContext {
    pub simulated_user: SimulatedUser,
}

impl TestSigningDriversContext {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

impl IsUseFactorSourceDriversContext for TestSigningDriversContext {
    fn driver_for_factor_source_kind(&self, kind: FactorSourceKind) -> UseFactorSourceDriver {
        match kind {
            FactorSourceKind::Device => UseFactorSourceDriver::parallel_batch(Arc::new(
                TestParallelBatchSigningDriver::new(self.simulated_user.clone()),
            )),
            FactorSourceKind::Arculus => UseFactorSourceDriver::serial_single(Arc::new(
                TestSerialSingleSigningDriver::new(self.simulated_user.clone()),
            )),
            _ => UseFactorSourceDriver::serial_batch(Arc::new(TestSerialBatchSigningDriver::new(
                self.simulated_user.clone(),
            ))),
        }
    }
}

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
        request: ParallelBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>> {
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

pub struct TestSerialBatchSigningDriver {
    simulated_user: SimulatedUser,
}

impl TestSerialBatchSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[async_trait]
impl IsTestUseFactorSourcesDriver for TestSerialBatchSigningDriver {
    fn simulated_user(&self) -> SimulatedUser {
        self.simulated_user.clone()
    }
}

#[async_trait]
impl SerialBatchUseFactorSourceDriver for TestSerialBatchSigningDriver {
    async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse> {
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
                SignWithFactorSourceOrSourcesOutcome::signed(response)
            }
            SigningUserInput::Skip => SignWithFactorSourceOrSourcesOutcome::skipped_factor_source(
                request.input.factor_source_id,
            ),
        }
    }
}
