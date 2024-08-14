use crate::prelude::*;

/// Empty marker protocol
pub trait IsUseFactorSourcesDriver {}

pub enum UseFactorSourceDriver {
    ParallelBatch(Arc<dyn ParallelBatchUseFactorSourcesDriver>),
    SerialBatch(Arc<dyn SerialBatchUseFactorSourceDriver>),
    SerialSingle(Arc<dyn SerialSingleUseFactorSourceDriver>),
}

impl UseFactorSourceDriver {
    pub fn parallel_batch(driver: Arc<dyn ParallelBatchUseFactorSourcesDriver>) -> Self {
        Self::ParallelBatch(driver)
    }

    pub fn serial_batch(driver: Arc<dyn SerialBatchUseFactorSourceDriver>) -> Self {
        Self::SerialBatch(driver)
    }

    pub fn serial_single(driver: Arc<dyn SerialSingleUseFactorSourceDriver>) -> Self {
        Self::SerialSingle(driver)
    }
}
