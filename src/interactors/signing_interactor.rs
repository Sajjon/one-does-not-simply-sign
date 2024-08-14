use crate::prelude::*;

/// An interactor which can sign transactions - either in parallel or serially.
pub enum SigningInteractor {
    ParallelBatch(Arc<dyn ParallelBatchUseFactorSourcesDriver>),
    SerialBatch(Arc<dyn SignWithFactorSerialInteractor>),
}

impl SigningInteractor {
    pub fn parallel_batch(interactor: Arc<dyn ParallelBatchUseFactorSourcesDriver>) -> Self {
        Self::ParallelBatch(interactor)
    }

    pub fn serial_batch(interactor: Arc<dyn SignWithFactorSerialInteractor>) -> Self {
        Self::SerialBatch(interactor)
    }
}
