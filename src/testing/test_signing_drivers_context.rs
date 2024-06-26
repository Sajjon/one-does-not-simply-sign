use crate::prelude::*;

pub struct TestSigningDriversContext {
    pub simulated_user: SimulatedUser,
}

impl TestSigningDriversContext {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

impl IsSigningDriversContext for TestSigningDriversContext {
    fn driver_for_factor_source_kind(&self, kind: FactorSourceKind) -> SigningDriver {
        match kind {
            FactorSourceKind::Device => SigningDriver::parallel_batch(Arc::new(
                TestParallelBatchSigningDriver::new(self.simulated_user.clone()),
            )),
            FactorSourceKind::Arculus => SigningDriver::serial_single(Arc::new(
                TestSerialSingleSigningDriver::new(self.simulated_user.clone()),
            )),
            _ => SigningDriver::serial_batch(Arc::new(TestSerialBatchSigningDriver::new(
                self.simulated_user.clone(),
            ))),
        }
    }
}

pub struct TestParallelBatchSigningDriver {
    pub simulated_user: SimulatedUser,
}

impl TestParallelBatchSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[async_trait]
impl ParallelBatchSigningDriver for TestParallelBatchSigningDriver {
    async fn sign(
        &self,
        request: ParallelBatchSigningRequest,
    ) -> SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse> {
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
                                x.owned_factor_instances()
                                    .iter()
                                    .map(|y| {
                                        HDSignature::new(
                                            x.intent_hash().clone(),
                                            Signature,
                                            y.clone(),
                                        )
                                    })
                                    .collect_vec()
                            })
                            .collect::<IndexSet<HDSignature>>();
                        (*k, value)
                    })
                    .collect::<IndexMap<FactorSourceID, IndexSet<HDSignature>>>();

                SignWithFactorSourceOrSourcesOutcome::signed(BatchSigningResponse::new(signatures))
            }

            SigningUserInput::Skip => SignWithFactorSourceOrSourcesOutcome::skipped(
                request
                    .per_factor_source
                    .keys()
                    .cloned()
                    .collect::<IndexSet<_>>(),
            ),
        }
    }
}

pub struct TestSerialSingleSigningDriver {
    pub simulated_user: SimulatedUser,
}

impl TestSerialSingleSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[async_trait]
impl SerialSingleSigningDriver for TestSerialSingleSigningDriver {
    async fn sign(
        &self,
        request: SerialSingleSigningRequestFull,
    ) -> SignWithFactorSourceOrSourcesOutcome<HDSignature> {
        match self
            .simulated_user
            .sign_or_skip(request.invalid_transactions_if_skipped)
        {
            SigningUserInput::Sign => {
                SignWithFactorSourceOrSourcesOutcome::signed(HDSignature::new(
                    request.input.intent_hash,
                    Signature,
                    request.input.owned_factor_instance,
                ))
            }
            SigningUserInput::Skip => SignWithFactorSourceOrSourcesOutcome::skipped_factor_source(
                request.input.factor_source_id,
            ),
        }
    }
}

pub struct TestSerialBatchSigningDriver {
    pub simulated_user: SimulatedUser,
}

impl TestSerialBatchSigningDriver {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

#[async_trait]
impl SerialBatchSigningDriver for TestSerialBatchSigningDriver {
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
                        let key = r.factor_source_id();
                        let value = r
                            .owned_factor_instances()
                            .iter()
                            .map(|f| {
                                HDSignature::new(r.intent_hash().clone(), Signature, f.clone())
                            })
                            .collect::<IndexSet<_>>();
                        (*key, value)
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
