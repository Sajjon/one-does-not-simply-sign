mod types;

/// A coordinator which gathers signatures from several factor sources of different
/// kinds for many transactions and for potentially multiple derivation paths
/// for each transaction.
pub struct SignaturesBuildingCoordinator;

/// Typically this would be one driver per factor source kind, but 
/// we make some assumptions here about us having a shared driver
/// for all kinds.
/// 
/// Most FactorSourceKinds does in fact NOT support parallel signing,
/// i.e. signing using multiple factors sources at once, but some do,
/// typically the DeviceFactorSource does, i.e. we can load multiple 
/// mnemonics from secure storage in one go and sign with all of them 
/// "in parallel".
/// 
/// This is a bit of a misnomer, as we don't actually sign in parallel,
/// but rather we iterate through all mnemonics and sign the 2D-batch 
/// payload with each of them in sequence. By 2D batch payload we mean
/// to sign multiple transactions each with multiple derivation paths.
pub struct ParallelSigningDriver;

pub mod prelude {
    pub use crate::types::*;

    pub use indexmap::*;
    pub use indexset::*;
}

pub use prelude::*;

mod tests {
    #[test]
    fn test() {

        
    }
}