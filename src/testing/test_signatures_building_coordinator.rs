use crate::prelude::*;

impl SignaturesBuildingCoordinator {
    pub fn new_test(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = TransactionIntent>,
        simulated_user: SimulatedUser,
    ) -> Self {
        Self::new(
            all_factor_sources_in_profile.into_iter().collect(),
            transactions.into_iter().collect(),
            Arc::new(TestSigningDriversContext::new(simulated_user)),
        )
    }
    pub fn test_prudent_with_factors(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::new_test(
            all_factor_sources_in_profile,
            transactions,
            SimulatedUser::Prudent,
        )
    }

    pub fn test_prudent(transactions: impl IntoIterator<Item = TransactionIntent>) -> Self {
        Self::test_prudent_with_factors(FactorSource::all(), transactions)
    }

    pub fn test_lazy_sign_minimum_with_factors(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::new_test(
            all_factor_sources_in_profile,
            transactions,
            SimulatedUser::Lazy(Laziness::SignMinimum),
        )
    }

    pub fn test_lazy_sign_minimum(
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::test_lazy_sign_minimum_with_factors(FactorSource::all(), transactions)
    }

    pub fn test_lazy_always_skip_with_factors(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::new_test(
            all_factor_sources_in_profile,
            transactions,
            SimulatedUser::Lazy(Laziness::AlwaysSkip),
        )
    }

    pub fn test_lazy_always_skip(
        transactions: impl IntoIterator<Item = TransactionIntent>,
    ) -> Self {
        Self::test_lazy_always_skip_with_factors(FactorSource::all(), transactions)
    }
}
