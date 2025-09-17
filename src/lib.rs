#![no_std]

pub mod process;

mod alloc;
pub mod host;
pub mod io;
mod panic;

pub use picolibc_macros::{host, main};
#[doc(hidden)]
pub use picolibc_sys as sys;
