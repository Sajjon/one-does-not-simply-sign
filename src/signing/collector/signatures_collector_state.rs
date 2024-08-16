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

impl IsFactorOutputCollectorState for SignaturesCollectorState {
    /// If all transactions already would fail, or if all transactions already are done, then
    /// no point in continuing.
    ///
    /// `Ok(true)` means "continue", `Ok(false)` means "stop, we are done". `Err(_)` means "stop, we have failed".
    fn continue_if_necessary(&self) -> Result<bool> {
        self.petitions.borrow().continue_if_necessary()
    }
}
