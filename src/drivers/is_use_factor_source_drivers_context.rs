use crate::prelude::*;

pub trait SignatureCollectingInteractors {
    fn driver_for_factor_source_kind(&self, kind: FactorSourceKind) -> UseFactorSourceDriver;
}
