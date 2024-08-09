#![allow(unused)]
#![allow(incomplete_features)]
#![feature(inherent_associated_types)]
#![allow(clippy::module_inception)]

mod fia;
mod support;
mod use_factor_source;

pub mod prelude {
    pub use crate::fia::*;
    pub use crate::support::*;
    pub use crate::use_factor_source::*;

    pub use std::{
        borrow::Borrow,
        cell::{Ref, RefCell, RefMut},
        collections::HashMap,
        collections::HashSet,
        hash::Hash,
        marker::PhantomData,
    };
}

pub use prelude::*;
