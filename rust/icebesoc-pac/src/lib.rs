#![no_std]

// Pull all the PAC stuff into our local namespace.
pub mod pac;

// This allows this library to re-export everything within `pac`.
pub use pac::*;

// These are all not exported from the main crate, but are pub when
// accessed through pac::generic.
pub use pac::generic::{Readable, Reg, ResetValue, Writable, R, W};
