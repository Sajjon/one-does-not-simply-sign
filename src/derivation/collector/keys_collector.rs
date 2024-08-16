use crate::prelude::*;

pub struct KeysCollectorDependencies;
pub struct KeysCollectorState;

/// A coordinator which gather derived PublicKeys from several factor sources
/// of different kinds, in increasing friction order, at many derivation paths.
///
/// By increasing friction order we mean, the quickest and easiest to use FactorSourceKind
/// is last; namely `DeviceFactorSource`, and the most tedious FactorSourceKind is
/// first; namely `LedgerFactorSource`, which user might also lack access to.
///
pub type KeysCollector = FactorInstancesCollector<KeysCollectorDependencies, KeysCollectorState>;
