#![allow(unused)]

use std::ops::Range;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeySpace {
    Unsecurified,
    Securified,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateNextDerivationPathRequest {
    pub factor_source_id: FactorSourceID,
    pub network_id: NetworkID,
    pub key_kind: CAP26KeyKind,
    pub entity_kind: CAP26EntityKind,
    pub key_space: KeySpace,
}

impl CreateNextDerivationPathRequest {
    pub fn new(
        factor_source_id: FactorSourceID,
        network_id: NetworkID,
        key_kind: CAP26KeyKind,
        entity_kind: CAP26EntityKind,
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
    ) -> HDPathComponent;

    fn next_derivation_index_for(
        &self,
        factor_source_id: FactorSourceID,
        network_id: NetworkID,
        key_kind: CAP26KeyKind,
        entity_kind: CAP26EntityKind,
        key_space: KeySpace,
    ) -> HDPathComponent {
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
        key_kind: CAP26KeyKind,
        entity_kind: CAP26EntityKind,
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
