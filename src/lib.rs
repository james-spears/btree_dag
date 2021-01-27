#![no_std]
extern crate alloc;

/// `error` module contains the definition of the Error struct.
pub mod error;

mod dag;
pub use dag::*;

