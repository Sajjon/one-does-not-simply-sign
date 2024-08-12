#![allow(unused)]
#![allow(incomplete_features)]
#![feature(inherent_associated_types)]
#![allow(clippy::module_inception)]

mod fia;
mod sargon_types;
mod use_factor_source;

mod testing;

pub mod prelude {
    pub use crate::fia::*;
    pub use crate::sargon_types::*;
    pub use crate::use_factor_source::*;

    pub use crate::testing::*;

    pub use std::{
        borrow::Borrow,
        cell::{Ref, RefCell, RefMut},
        collections::HashMap,
        collections::HashSet,
        hash::Hash,
        marker::PhantomData,
        time::SystemTime,
    };

    pub use derive_getters::Getters;
    pub use indexmap::IndexSet;
    pub use itertools::*;
    pub use uuid::Uuid;
}

pub use prelude::*;
