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

    type Request = SerialBatchSigningRequest;

    fn do_sign(&self, request: &Self::Request) -> IndexMap<FactorSourceID, IndexSet<HDSignature>> {
        self.do_do_sign(request.input.per_transaction.clone())
    }
}

pub trait SharedRequest {
    fn factor_source_ids(&self) -> IndexSet<FactorSourceID>;

    fn invalid_transactions_if_skipped(&self) -> IndexSet<InvalidTransactionIfSkipped>;
}
impl SharedRequest for SerialBatchSigningRequest {
    fn factor_source_ids(&self) -> IndexSet<FactorSourceID> {
        IndexSet::from_iter([self.input.factor_source_id])
    }
    fn invalid_transactions_if_skipped(&self) -> IndexSet<InvalidTransactionIfSkipped> {
        self.invalid_transactions_if_skipped
            .clone()
            .into_iter()
            .collect()
    }
}

impl SharedRequest for ParallelBatchSigningRequest {
    fn factor_source_ids(&self) -> IndexSet<FactorSourceID> {
        self.per_factor_source.keys().cloned().collect()
    }
    fn invalid_transactions_if_skipped(&self) -> IndexSet<InvalidTransactionIfSkipped> {
        self.invalid_transactions_if_skipped.clone()
    }
}

#[async_trait]
impl SignWithFactorSerialInteractor for TestSigningSerialInteractor {
    async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>> {
        self.shared_sign(request)
    }
}
