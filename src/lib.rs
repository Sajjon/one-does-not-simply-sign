#![allow(unused)]

pub mod fia;
pub mod support;

pub mod prelude {
    pub use crate::fia::*;
    pub use crate::support::*;

    pub use std::{collections::HashMap, hash::Hash, marker::PhantomData};
}

pub use prelude::*;
