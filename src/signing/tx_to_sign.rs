use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, std::hash::Hash)]
pub struct TXToSign {
    pub intent_hash: IntentHash,
    entities_requiring_auth: Vec<AccountOrPersona>, // should be a set but Sets are not `Hash`.
}

impl TXToSign {
    pub fn new(
        entities_requiring_auth: impl IntoIterator<Item = impl Into<AccountOrPersona>>,
    ) -> Self {
        Self {
            intent_hash: IntentHash::generate(),
            entities_requiring_auth: entities_requiring_auth
                .into_iter()
                .map(|i| i.into())
                .collect_vec(),
        }
    }

    pub fn entities_requiring_auth(&self) -> IndexSet<AccountOrPersona> {
        self.entities_requiring_auth.clone().into_iter().collect()
    }
}
