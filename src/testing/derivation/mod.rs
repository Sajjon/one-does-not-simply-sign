use crate::prelude;

pub struct TestDerivationInteractors;

impl KeysCollectingInteractors for TestDerivationInteractors {}

impl KeysCollector {
    pub fn new_test(
        all_factor_sources_in_profile: impl IntoIterator<Item = FactorSource>,
        scenario: DerivationScenario,
        used: UsedDerivationIndices,
    ) -> Self {
        // Self::new(
        //     all_factor_sources_in_profile.into_iter().collect(),
        //     transactions.into_iter().collect(),
        //     Arc::new(TestSignatureCollectingInteractors::new(simulated_user)),
        // )
        Self::new
    }
}
