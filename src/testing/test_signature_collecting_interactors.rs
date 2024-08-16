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
    fn interactor_for(&self, kind: FactorSourceKind) -> InteractorParallelOrSerial {
        match kind {
            FactorSourceKind::Device => InteractorParallelOrSerial::parallel(Arc::new(
                TestSigningParallelInteractor::new(self.simulated_user.clone()),
            )),
            _ => InteractorParallelOrSerial::serial(Arc::new(TestSigningSerialInteractor::new(
                self.simulated_user.clone(),
            ))),
        }
    }
}
