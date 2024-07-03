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
