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

#[async_trait::async_trait]
impl SignWithFactorBaseInteractor<SerialBatchSigningRequest> for TestSigningSerialInteractor {
    async fn sign(
        &self,
        request: SerialBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>> {
        self.shared_sign(request)
    }
}
