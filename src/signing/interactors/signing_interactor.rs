use crate::prelude::*;

/// An interactor which can sign transactions - either in parallel or serially.
pub enum InteractorParallelOrSerial<P, S>
where
    P: UseFactorParallelInteractor,
    S: UseFactorParallelInteractor,
{
    Parallel(P),
    Serial(S),
}

impl InteractorParallelOrSerial {
    pub fn parallel(interactor: Arc<dyn SignWithFactorParallelInteractor>) -> Self {
        Self::Parallel(interactor)
    }

    pub fn serial(interactor: Arc<dyn SignWithFactorSerialInteractor>) -> Self {
        Self::Serial(interactor)
    }
}
