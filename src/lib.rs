//! Question: Is there any difference between BatchSigningDrivers and
//! SingleSigningDrivers other than the fact that BatchSigningDerivers can sign
//! many transactions with many derivations paths at once?

mod existing_types_in_sargon;
mod testing;
mod types;

pub mod prelude {
    pub use crate::existing_types_in_sargon::*;
    pub use crate::testing::*;
    pub use crate::types::*;

    pub use async_trait::async_trait;
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
