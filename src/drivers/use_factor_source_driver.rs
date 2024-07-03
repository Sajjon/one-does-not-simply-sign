use crate::prelude::*;

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

    pub async fn did_fail_ask_if_retry(&self, factor_source_ids: IndexSet<FactorSourceID>) -> bool {
        match self {
            UseFactorSourceDriver::ParallelBatch(driver) => {
                driver.did_fail_ask_if_retry(factor_source_ids).await
            }
            UseFactorSourceDriver::SerialBatch(driver) => {
                driver.did_fail_ask_if_retry(factor_source_ids).await
            }
            UseFactorSourceDriver::SerialSingle(driver) => {
                driver.did_fail_ask_if_retry(factor_source_ids).await
            }
        }
    }
}
