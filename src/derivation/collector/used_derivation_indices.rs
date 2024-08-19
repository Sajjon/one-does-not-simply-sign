#![allow(unused)]

use std::ops::Range;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeySpace {
    Unsecurified,
    Securified,
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

#[derive(Clone, Debug)]
pub struct KeysOfEntityKindInKeySpaceCollection {
    key_space: KeySpace,
    transaction_signing: RefCell<IndexSet<FactorInstance>>,
    authentication_signing: RefCell<IndexSet<FactorInstance>>,
}
impl KeysOfEntityKindInKeySpaceCollection {
    pub fn for_key_kind(&self, key_kind: &KeyKind) -> IndexSet<FactorInstance> {
        match key_kind {
            KeyKind::Rola => self.authentication_signing.borrow().clone(),
            KeyKind::T9n => self.transaction_signing.borrow().clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeysOfEntityKindCollection {
    entity_kind: EntityKind,
    unsecurified: RefCell<KeysOfEntityKindInKeySpaceCollection>,
    securified: RefCell<KeysOfEntityKindInKeySpaceCollection>,
}
impl KeysOfEntityKindCollection {
    pub fn for_key_space(&self, key_space: &KeySpace) -> KeysOfEntityKindInKeySpaceCollection {
        match key_space {
            KeySpace::Securified => self.securified.borrow().clone(),
            KeySpace::Unsecurified => self.unsecurified.borrow().clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeysOnNetworkCollection {
    network_id: NetworkID,
    accounts: RefCell<KeysOfEntityKindCollection>,
    identities: RefCell<KeysOfEntityKindCollection>,
}
impl KeysOnNetworkCollection {
    pub fn for_entity_kind(&self, entity_kind: &EntityKind) -> KeysOfEntityKindCollection {
        match entity_kind {
            EntityKind::Account => self.accounts.borrow().clone(),
            EntityKind::Identity => self.identities.borrow().clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeysCollection {
    factor_source_id: FactorSourceID,
    networks: RefCell<IndexMap<NetworkID, KeysOnNetworkCollection>>,
}
impl KeysCollection {
    pub fn on_network(&self, network_id: &NetworkID) -> Option<KeysOnNetworkCollection> {
        self.networks.borrow().get(network_id).cloned()
    }
}

#[derive(Default, Clone, Debug)]
pub struct DefaultUsedDerivationIndices {
    collections: RefCell<IndexMap<FactorSourceID, KeysCollection>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateNextDerivationPathRequest {
    factor_source_id: FactorSourceID,
    network_id: NetworkID,
    key_kind: KeyKind,
    entity_kind: EntityKind,
    key_space: KeySpace,
}

impl KeySpace {
    pub const SPLIT: u32 = 0x4000_0000;
    pub const HARDENED: u32 = 0x8000_0000;
    pub fn range(&self) -> Range<DerivationIndex> {
        match self {
            Self::Unsecurified => 0..Self::SPLIT,
            Self::Securified => Self::SPLIT..Self::HARDENED,
        }
    }
}
impl FactorInstance {
    pub fn fulfills_request(&self, request: &CreateNextDerivationPathRequest) -> bool {
        request.matches_instance(self)
    }
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

impl UsedDerivationIndices for DefaultUsedDerivationIndices {
    fn next_derivation_index_with_request(
        &self,
        request: CreateNextDerivationPathRequest,
    ) -> DerivationIndex {
        let mut next = Option::<DerivationIndex>::None;
        if let Some(ref collection) = self
            .collections
            .borrow_mut()
            .get_mut(&request.factor_source_id)
        {
            if let Some(on_network) = collection.on_network(&request.network_id) {
                let nxt = on_network
                    .for_entity_kind(&request.entity_kind)
                    .for_key_space(&request.key_space)
                    .for_key_kind(&request.key_kind)
                    .iter()
                    .find(|instance| instance.fulfills_request(&request))
                    .map(|instance| instance.hd_public_key.derivation_path.index)
                    .map(|prev_index| prev_index + 1)
                    .unwrap_or(0);

                next = Some(nxt);
            }
        };

        match next {
            Some(index) => index,
            None => request.key_space.range().start,
        }
    }
}
