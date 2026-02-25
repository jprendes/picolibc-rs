#![no_std]

extern crate alloc;

pub mod process;

mod allocator;
mod emutls;
mod panic;
mod stubs;
pub mod host;
pub mod io;
pub mod thread;
pub mod time;

pub use picolibc_macros::{host, main};

#[doc(hidden)]
pub use picolibc_sys as sys;
