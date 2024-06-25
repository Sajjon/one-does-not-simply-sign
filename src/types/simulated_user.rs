#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SigningUserInput {
    Sign,
    Skip,
}

#[cfg(test)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SimulatedUser {
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
#[cfg(test)]
impl SimulatedUser {
    pub fn lazy_always_skip() -> Self {
        Self::Lazy(Laziness::AlwaysSkip)
    }
    /// Skips only if `invalid_tx_if_skipped` is empty
    pub fn lazy_sign_minimum() -> Self {
        Self::Lazy(Laziness::SignMinimum)
    }
}

#[cfg(test)]
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

#[cfg(test)]
impl SimulatedUser {
    pub fn sign_or_skip(
        &self,
        invalid_tx_if_skipped: impl IntoIterator<Item = crate::prelude::InvalidTransactionIfSkipped>,
    ) -> SigningUserInput {
        use rand::prelude::*;
        let invalid_tx_if_skipped = invalid_tx_if_skipped
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        match self {
            SimulatedUser::Prudent => SigningUserInput::Sign,
            SimulatedUser::Lazy(laziness) => match laziness {
                Laziness::AlwaysSkip => SigningUserInput::Skip,
                Laziness::SignMinimum => {
                    if invalid_tx_if_skipped.is_empty() {
                        SigningUserInput::Skip
                    } else {
                        SigningUserInput::Sign
                    }
                }
            },
            SimulatedUser::Random => {
                let mut rng = rand::thread_rng();
                let num: f64 = rng.gen(); // generates a float between 0 and 1
                if num > 0.5 {
                    SigningUserInput::Skip
                } else {
                    SigningUserInput::Sign
                }
            }
        }
    }
}
