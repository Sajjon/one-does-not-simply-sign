use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    collections::HashMap,
};

use indexmap::IndexSet;

use crate::FactorSourceID;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigningUserInput {
    Sign,
    Skip,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulatedUser {
    mode: SimulatedUserMode,
    /// `None` means never fail / retry
    retry: Option<RefCell<SimulatedUserRetries>>,
}

impl SimulatedUser {
    pub fn new(mode: SimulatedUserMode, retry: impl Into<Option<SimulatedUserRetries>>) -> Self {
        Self {
            mode,
            retry: retry.into().map(|r| RefCell::new(r)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulatedUserRetries {
    max_retries: usize,
    retries: RefCell<HashMap<FactorSourceID, SimulatedUserRetry>>,

    /// `0` means "never fail", `1` means fail first time only, succeed second time.
    /// `2` means fail first two times, succeed third time, and so on.
    simulated_failures: HashMap<FactorSourceID, usize>,
}
impl SimulatedUserRetries {
    pub const DEFAULT_RETRY_COUNT: usize = 2;
    pub fn with_details(
        max_retries: usize,
        retries: HashMap<FactorSourceID, SimulatedUserRetry>,
        simulated_failures: HashMap<FactorSourceID, usize>,
    ) -> Self {
        Self {
            max_retries,
            retries: retries.into(),
            simulated_failures,
        }
    }

    pub fn with_simulated_failures(
        max_retries: usize,
        failures: impl IntoIterator<Item = (FactorSourceID, usize)>,
    ) -> Self {
        Self::with_details(
            max_retries,
            HashMap::new(),
            HashMap::from_iter(failures.into_iter()),
        )
    }

    pub fn new() -> Self {
        Self::with_details(Self::DEFAULT_RETRY_COUNT, HashMap::new(), HashMap::new())
    }

    /// returns `true` if we should retry, which updates increases retry count
    fn single_retry_if_needed(&self, factor_source_id: &FactorSourceID) -> bool {
        let retries = self.retries.borrow_mut();
        let Some(retry) = retries.get(factor_source_id) else {
            return false;
        };
        if retry.retry_count() >= self.max_retries {
            return false;
        }
        retry.retry();
        return true;
    }

    /// returns `true` if we should fail, which updates increases failure count
    fn single_simulate_failure_if_needed(&self, factor_source_id: &FactorSourceID) -> bool {
        let retries = self.retries.borrow_mut();
        let Some(retry) = retries.get(factor_source_id) else {
            return false;
        };
        let Some(failure) = self.simulated_failures.get(factor_source_id) else {
            return false;
        };
        if *failure < retry.failures.get() {
            return false;
        }
        retry.fail();
        return true;
    }

    /// If needed, retries ALL factor sources or NONE.
    pub fn retry_if_needed(&self, factor_source_ids: IndexSet<FactorSourceID>) -> bool {
        factor_source_ids
            .into_iter()
            .map(|id| self.single_retry_if_needed(&id))
            .any(|x| x)
    }

    /// If needed, simulates failure for ALL factor sources or NONE.
    pub fn simulate_failure_if_needed(&self, factor_source_ids: IndexSet<FactorSourceID>) -> bool {
        factor_source_ids
            .into_iter()
            .map(|id| self.single_simulate_failure_if_needed(&id))
            .any(|x| x)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SimulatedUserRetry {
    failures: Cell<usize>,
    retries: Cell<usize>,
}
impl SimulatedUserRetry {
    pub fn fail(&self) {
        self.failures.set(self.failures.get() + 1);
    }
    pub fn retry(&self) {
        self.retries.set(self.retries.get() + 1);
    }
    pub fn retry_count(&self) -> usize {
        self.retries.get()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimulatedUserMode {
    /// Emulation of a "prudent" user, that signs with all factors sources, i.e.
    /// she never ever "skips" a factor source
    Prudent,

    /// Emulation of a "lazy" user, that skips signing with as many factor
    /// sources as possible.
    Lazy(Laziness),

    /// Emulation of a "random" user, that skips signing some factor sources
    ///  at random.
    Random,
}

impl SimulatedUserMode {
    pub fn lazy_always_skip() -> Self {
        Self::Lazy(Laziness::AlwaysSkip)
    }

    /// Skips only if `invalid_tx_if_skipped` is empty
    pub fn lazy_sign_minimum() -> Self {
        Self::Lazy(Laziness::SignMinimum)
    }
}

impl SimulatedUser {
    pub fn prudent() -> Self {
        Self {
            mode: SimulatedUserMode::Prudent,
            retry: None,
        }
    }
    pub fn lazy_always_skip_no_fail() -> Self {
        Self::new(SimulatedUserMode::lazy_always_skip(), None)
    }

    /// Skips only if `invalid_tx_if_skipped` is empty, if retry_on_failure
    pub fn lazy_sign_minimum(
        simulated_failures: impl IntoIterator<Item = (FactorSourceID, usize)>,
    ) -> Self {
        Self::new(
            SimulatedUserMode::lazy_sign_minimum(),
            SimulatedUserRetries::with_simulated_failures(
                SimulatedUserRetries::DEFAULT_RETRY_COUNT,
                simulated_failures,
            ),
        )
    }
}

unsafe impl Sync for SimulatedUser {}

/// A very lazy user that defers all boring work such as signing stuff for as long
/// as possible. Ironically, this sometimes leads to user signing more than she
/// actually needs. For example, if the user has a Securified Account with threshold
/// and override factors, she actually needs to sign with a single override
/// factor. But since user is so lazy, she defers signing with that override
/// factor if prompted for it first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Laziness {
    SignMinimum,
    AlwaysSkip,
}

impl SimulatedUser {
    pub fn sign_or_skip(
        &self,
        invalid_tx_if_skipped: impl IntoIterator<Item = crate::prelude::InvalidTransactionIfSkipped>,
    ) -> SigningUserInput {
        let invalid_tx_if_skipped = invalid_tx_if_skipped
            .into_iter()
            .collect::<std::collections::HashSet<_>>();

        if self.be_prudent(|| invalid_tx_if_skipped.is_empty()) {
            SigningUserInput::Sign
        } else {
            SigningUserInput::Skip
        }
    }

    pub fn retry_if_needed(&self, factor_source_ids: IndexSet<FactorSourceID>) -> bool {
        if let Some(retry) = &self.retry {
            retry.borrow_mut().retry_if_needed(factor_source_ids)
        } else {
            false
        }
    }

    pub fn simulate_failure_if_needed(&self, factor_source_ids: IndexSet<FactorSourceID>) -> bool {
        if let Some(retry) = &self.retry {
            retry
                .borrow_mut()
                .simulate_failure_if_needed(factor_source_ids)
        } else {
            false
        }
    }

    fn be_prudent<F>(&self, is_prudent: F) -> bool
    where
        F: Fn() -> bool,
    {
        use rand::prelude::*;

        match &self.mode {
            SimulatedUserMode::Prudent => true,
            SimulatedUserMode::Lazy(laziness) => match laziness {
                Laziness::AlwaysSkip => false,
                Laziness::SignMinimum => is_prudent(),
            },
            SimulatedUserMode::Random => {
                let mut rng = rand::thread_rng();
                let num: f64 = rng.gen(); // generates a float between 0 and 1
                num < 0.5
            }
        }
    }
}
