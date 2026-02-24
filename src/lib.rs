#![no_std]

extern crate alloc;

pub mod process;

mod allocator;
pub mod host;
pub mod io;
mod panic;
mod emutls;
pub mod thread;

pub use picolibc_macros::{host, main};
#[doc(hidden)]
pub use picolibc_sys as sys;
