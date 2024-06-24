//! Question: Is there any difference between BatchSigningDrivers and
//! SingleSigningDrivers other than the fact that BatchSigningDerivers can sign
//! many transactions with many derivations paths at once?

mod types;

pub mod prelude {
    pub use crate::types::*;

    pub use actix_rt::*;
    pub use async_trait::async_trait;
    pub use indexmap::*;
    pub use indexset::*;
    pub use itertools::Itertools;
    pub use std::time::SystemTime;
    pub use uuid::Uuid;

    pub use std::{
        collections::{HashMap, HashSet},
        sync::Arc,
    };
}

pub use prelude::*;

mod tests {
    #[test]
    fn test() {}
}
