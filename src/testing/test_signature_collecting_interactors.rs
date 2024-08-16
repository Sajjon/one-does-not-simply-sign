use crate::prelude::*;

pub struct TestSignatureCollectingInteractors {
    pub simulated_user: SimulatedUser,
}

impl TestSignatureCollectingInteractors {
    pub fn new(simulated_user: SimulatedUser) -> Self {
        Self { simulated_user }
    }
}

impl InteractorsContext<SignWithFactorParallelInteractor, SignWithFactorSerialInteractor>
    for TestSignatureCollectingInteractors
{
    fn interactor_for(&self, kind: FactorSourceKind) -> SigningInteractor {
        match kind {
            FactorSourceKind::Device => SigningInteractor::parallel(Arc::new(
                TestSigningParallelInteractor::new(self.simulated_user.clone()),
            )),
            _ => SigningInteractor::serial(Arc::new(TestSigningSerialInteractor::new(
                self.simulated_user.clone(),
            ))),
        }
    }
}
