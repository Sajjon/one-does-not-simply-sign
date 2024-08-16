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
}
