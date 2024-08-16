use crate::prelude::*;

pub struct SignaturesCollectorState {
    pub(crate) petitions: RefCell<Petitions>,
}

impl SignaturesCollectorState {
    pub(crate) fn new(petitions: Petitions) -> Self {
        Self {
            petitions: RefCell::new(petitions),
        }
    }
}
