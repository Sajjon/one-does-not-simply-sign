use crate::prelude::*;

pub struct TestSignatureCollectingInteractors {
    pub simulated_user: SimulatedUser,
}

impl TestSignatureCollectingInteractors {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

impl SignatureCollectingInteractors for TestSignatureCollectingInteractors {
    fn interactor_for(&self, kind: FactorSourceKind) -> SigningInteractor {
        match kind {
            FactorSourceKind::Device => SigningInteractor::parallel_batch(Arc::new(
                TestParallelBatchSigningDriver::new(self.simulated_user.clone()),
            )),
            _ => SigningInteractor::serial_batch(Arc::new(TestSerialInteractor::new(
                self.simulated_user.clone(),
            ))),
        }
    }
}
