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

    pub fn matches_instance(&self, instance: &FactorInstance) -> bool {
        self.matches_path(
            &instance.hd_public_key.derivation_path,
            &instance.factor_source_id,
        )
    }
    pub fn matches_path(&self, path: &DerivationPath, factor_source_id: &FactorSourceID) -> bool {
        if !(path.entity_kind == self.entity_kind
            && path.key_kind == self.key_kind
            && self.factor_source_id == *factor_source_id)
        {
            return false;
        }
        self.key_space.range().contains(&path.index)
    }
}

impl FactorInstance {
    pub fn fulfills_request(&self, request: &CreateNextDerivationPathRequest) -> bool {
        request.matches_instance(self)
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

    fn next_derivation_path_account_tx(
        &self,
        factor_source_id: FactorSourceID,
        network_id: NetworkID,
    ) -> DerivationPath {
        self.next_derivation_path(
            factor_source_id,
            network_id,
            KeyKind::T9n,
            EntityKind::Account,
            KeySpace::Unsecurified,
        )
    }
}
