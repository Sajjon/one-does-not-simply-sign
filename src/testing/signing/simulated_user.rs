use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    collections::HashMap,
    sync::{Arc, Mutex, RwLock},
};

use indexmap::IndexSet;

use crate::FactorSourceIDFromHash;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigningUserInput {
    Sign,
    Skip,
}

#[derive(Debug, Clone)]
pub struct SimulatedUser {
    mode: SimulatedUserMode,
    /// `None` means never failures
    failures: Option<SimulatedFailures>,
}

impl SimulatedUser {
    pub fn new(mode: SimulatedUserMode, failures: impl Into<Option<SimulatedFailures>>) -> Self {
        Self {
            mode,
            failures: failures.into(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SimulatedFailures {
    /// Set of FactorSources which should always fail.
    simulated_failures: IndexSet<FactorSourceIDFromHash>,
}
impl SimulatedFailures {
    pub fn with_details(simulated_failures: IndexSet<FactorSourceIDFromHash>) -> Self {
        Self { simulated_failures }
    }

    pub fn with_simulated_failures(
        failures: impl IntoIterator<Item = FactorSourceIDFromHash>,
    ) -> Self {
        Self::with_details(IndexSet::from_iter(failures))
    }

    /// If needed, simulates failure for ALL factor sources or NONE.
    pub fn simulate_failure_if_needed(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        factor_source_ids
            .into_iter()
            .all(|id| self.simulated_failures.contains(&id))
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
    pub fn prudent_no_fail() -> Self {
        Self::new(SimulatedUserMode::Prudent, None)
    }

    pub fn prudent_with_failures(simulated_failures: SimulatedFailures) -> Self {
        Self::new(SimulatedUserMode::Prudent, simulated_failures)
    }

    pub fn lazy_always_skip_no_fail() -> Self {
        Self::new(SimulatedUserMode::lazy_always_skip(), None)
    }

    /// Skips only if `invalid_tx_if_skipped` is empty
    /// (or if simulated failure for that factor source)
    pub fn lazy_sign_minimum(
        simulated_failures: impl IntoIterator<Item = FactorSourceIDFromHash>,
    ) -> Self {
        Self::new(
            SimulatedUserMode::lazy_sign_minimum(),
            SimulatedFailures::with_simulated_failures(simulated_failures),
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

        if self.be_prudent(|| !invalid_tx_if_skipped.is_empty()) {
            SigningUserInput::Sign
        } else {
            SigningUserInput::Skip
        }
    }

    pub fn simulate_failure_if_needed(
        &self,
        factor_source_ids: IndexSet<FactorSourceIDFromHash>,
    ) -> bool {
        if let Some(failures) = &self.failures {
            failures.simulate_failure_if_needed(factor_source_ids)
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
        }
    }
}
