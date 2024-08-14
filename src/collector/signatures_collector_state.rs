use crate::prelude::*;

pub(super) struct SignaturesCollectorState {
    pub(super) petitions: RefCell<Petitions>,
}
impl SignaturesCollectorState {
    pub fn new(petitions: Petitions) -> Self {
        Self {
            petitions: RefCell::new(petitions),
        }
    }
}
