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

    type Request = ParallelBatchSigningRequest;

    fn do_sign(&self, request: &Self::Request) -> IndexMap<FactorSourceID, IndexSet<HDSignature>> {
        request
            .per_factor_source
            .clone()
            .into_iter()
            .flat_map(|(_, v)| self.do_do_sign(v.per_transaction))
            .collect()
    }
}

#[async_trait]
impl SignWithFactorParallelInteractor for TestSigningParallelInteractor {
    async fn sign(
        &self,
        request: ParallelBatchSigningRequest,
    ) -> Result<SignWithFactorSourceOrSourcesOutcome<BatchSigningResponse>> {
        self.shared_sign(request)
    }
}
