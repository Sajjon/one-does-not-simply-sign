#![allow(unused)]

use std::ops::Range;

use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateNextDerivationPathRequest {
    pub factor_source_id: FactorSourceID,
    pub network_id: NetworkID,
    pub key_kind: KeyKind,
    pub entity_kind: EntityKind,
    pub key_space: KeySpace,
}

impl CreateNextDerivationPathRequest {
    pub fn new(
        factor_source_id: FactorSourceID,
        network_id: NetworkID,
        key_kind: KeyKind,
        entity_kind: EntityKind,
        key_space: KeySpace,
    ) -> Self {
        Self {
            factor_source_id,
            network_id,
            key_kind,
            entity_kind,
            key_space,
        }
    }
}

pub trait UsedDerivationIndices {
    fn next_derivation_index_with_request(
        &self,
        request: CreateNextDerivationPathRequest,
    ) -> DerivationIndex;

    fn next_derivation_index_for(
        &self,
        factor_source_id: FactorSourceID,
        network_id: NetworkID,
        key_kind: KeyKind,
        entity_kind: EntityKind,
        key_space: KeySpace,
    ) -> DerivationIndex {
        let request = CreateNextDerivationPathRequest::new(
            factor_source_id,
            network_id,
            key_kind,
            entity_kind,
            key_space,
        );
        self.next_derivation_index_with_request(request)
    }

    fn next_derivation_path(
        &self,
        factor_source_id: FactorSourceID,
        network_id: NetworkID,
        key_kind: KeyKind,
        entity_kind: EntityKind,
        key_space: KeySpace,
    ) -> DerivationPath {
        let index = self.next_derivation_index_for(
            factor_source_id,
            network_id,
            key_kind,
            entity_kind,
            key_space,
        );
        DerivationPath::new(network_id, entity_kind, key_kind, index)
    }
}
