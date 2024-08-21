use std::ops::Range;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeySpace {
    Unsecurified,
    Securified,
}

impl KeySpace {
    pub const SPLIT: u32 = 0x4000_0000;
    pub const HARDENED: u32 = 0x8000_0000;
    pub fn range(&self) -> Range<DerivationIndex> {
        match self {
            Self::Unsecurified => 0..Self::SPLIT,
            Self::Securified => Self::SPLIT..Self::HARDENED,
        }
    }
}
