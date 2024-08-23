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
    pub fn persona_by_address(&self, address: IdentityAddress) -> Result<Persona> {
        self.personas
            .get(&address)
            .ok_or(CommonError::UnknownPersona)
            .cloned()
    }
}

impl TransactionIntent {
    pub fn manifest_summary(&self) -> ManifestSummary {
        self.manifest.summary()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_address() {
        let account = AccountOrPersona::from(Account::sample());
        assert_eq!(account.address().to_string(), "acco_Alice")
    }

    #[test]
    fn persona_address() {
        let persona = AccountOrPersona::from(Persona::sample());
        assert_eq!(persona.address().to_string(), "ident_Alice")
    }
}
