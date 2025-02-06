#![feature(inline_const_pat)]

pub mod bus;
pub mod cpu;
pub mod ppu;
pub mod prelude;
#[cfg(test)]
pub(crate) use prelude::internal_macros::*;
