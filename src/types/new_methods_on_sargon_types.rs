use crate::prelude::*;

impl AccountOrPersona {
    pub fn address(&self) -> AddressOfAccountOrPersona {
        match self {
            Self::AccountEntity(a) => a.address().clone(),
            Self::PersonaEntity(p) => p.address().clone(),
        }
    }

    pub fn security_state(&self) -> EntitySecurityState {
        match self {
            Self::AccountEntity(a) => a.security_state.clone(),
            Self::PersonaEntity(p) => p.security_state.clone(),
        }
    }
}
