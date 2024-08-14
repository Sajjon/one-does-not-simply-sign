//! Question: Is there any difference between BatchSigningDrivers and
//! SingleSigningDrivers other than the fact that BatchSigningDerivers can sign
//! many transactions with many derivations paths at once?

mod drivers;
mod owned_types;
mod petition_types;
mod signatures_collecting_coordinator;
mod signatures_outcome_types;
mod testing;
mod types;

pub mod prelude {
    pub use crate::drivers::*;
    pub use crate::owned_types::*;
    pub use crate::petition_types::*;
    pub use crate::signatures_collecting_coordinator::*;
    pub use crate::signatures_outcome_types::*;
    pub use crate::testing::*;
    pub use crate::types::*;

    pub use async_trait::async_trait;
    pub use derive_getters::Getters;
    pub use indexmap::{IndexMap, IndexSet};
    pub use itertools::Itertools;
    pub use std::cell::RefCell;
    pub use std::time::SystemTime;
    pub use uuid::Uuid;

    pub use std::{
        collections::{BTreeMap, BTreeSet, HashMap, HashSet},
        sync::Arc,
    };
}

pub use prelude::*;
