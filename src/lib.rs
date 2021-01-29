#![no_std]
extern crate alloc;

/// `Error` type is re-exported from the separate btree_error crate.
pub type Error = btree_error::Error;

mod dag;
pub use dag::*;
