use crate::prelude::*;

/// A driver for factor source kinds which cannot sign multiple transactions
/// nor sign a single transaction with multiple keys (derivation paths).
///
/// Example of a Serial Single Signing Driver *might* be `Arculus` - we
/// do not yet know.
pub struct SerialSingleSigningDriver;
