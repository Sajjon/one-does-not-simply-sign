use crate::prelude::*;

/// A coordinator which gathers signatures from several factor sources of different
/// kinds for many transactions and for potentially multiple derivation paths
/// for each transaction.
pub struct SignaturesBuildingCoordinator;