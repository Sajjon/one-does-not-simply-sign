use crate::prelude::*;

pub trait IsTestInteractor: Sync {
    fn simulated_user(&self) -> SimulatedUser;

    fn should_simulate_failure(&self, factor_source_ids: IndexSet<FactorSourceID>) -> bool {
        self.simulated_user()
            .simulate_failure_if_needed(factor_source_ids)
    }
}
