use crate::prelude::*;

pub struct KeysCollectorState {
    pub(super) keyrings: RefCell<Keyrings>,
}

impl KeysCollectorState {
    pub fn new(keyrings: Keyrings) -> Self {
        Self {
            keyrings: RefCell::new(keyrings),
        }
    }

    pub(crate) fn process_batch_response(&self, response: BatchDerivationResponse) {
        self.keyrings.borrow_mut().process_batch_response(response)
    }
}
