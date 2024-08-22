use crate::prelude::*;

pub(super) struct SignaturesCollectorState {
    pub(super) petitions: RefCell<Petitions>,
}
impl SignaturesCollectorState {
    pub fn new(petitions: Petitions) -> Self {
        println!("petitions: {:?}", &petitions);
        Self {
            petitions: RefCell::new(petitions),
        }
    }
}
