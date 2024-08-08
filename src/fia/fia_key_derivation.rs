use crate::prelude::*;

/// Derives many public keys per FactorSource, e.g. used to `SecurityStructureOfFactorSources -> SecurityStructureOfFactorInstances`
pub type FiaKeyDerivation = FactorInstanceAccumulator<DeriveKeyID, DerivationPath, HDPublicKey>;

impl FiaKeyDerivation {
    pub fn new_batch_derive_public_keys(
        inputs: HashMap<FactorSourceID, HashMap<DeriveKeyID, Vec<DerivationPath>>>,
        factor_sources: Vec<FactorSource>,
    ) -> Result<Self> {
        Self::new(BatchUseFactorSourceRequest::new(inputs), factor_sources)
    }
}
