use crate::prelude::*;

pub trait IsUseFactorSourceDriversContext {
    fn driver_for_factor_source_kind(&self, kind: FactorSourceKind) -> SigningDriver;
}
