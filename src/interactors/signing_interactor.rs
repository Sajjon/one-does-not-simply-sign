use crate::prelude::*;

/// An interactor which can sign transactions - either in parallel or serially.
pub enum SigningInteractor {
    Parallel(Arc<dyn SignWithFactorParallelInteractor>),
    Serial(Arc<dyn SignWithFactorSerialInteractor>),
}

impl SigningInteractor {
    pub fn parallel(interactor: Arc<dyn SignWithFactorParallelInteractor>) -> Self {
        Self::Parallel(interactor)
    }

    pub fn serial(interactor: Arc<dyn SignWithFactorSerialInteractor>) -> Self {
        Self::Serial(interactor)
    }
}
