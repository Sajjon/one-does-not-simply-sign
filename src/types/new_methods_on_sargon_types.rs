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

impl Profile {
    pub fn persona_by_address(&self, _address: IdentityAddress) -> Result<Persona> {
        todo!()
    }
}

impl TransactionIntent {
    pub fn manifest_summary(&self) -> ManifestSummary {
        self.manifest.summary()
    }
}
