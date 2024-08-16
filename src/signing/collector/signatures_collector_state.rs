use crate::prelude::*;

pub struct SignaturesCollectorState {
    pub(super) petitions: RefCell<Petitions>,
}

impl SignaturesCollectorState {
    pub(crate) fn new(petitions: Petitions) -> Self {
        Self {
            petitions: RefCell::new(petitions),
        }
    }
}
