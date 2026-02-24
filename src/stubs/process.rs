use core::ffi::*;

use crate::host::HOST;

#[unsafe(no_mangle)]
extern "C" fn rust_start() -> ! {
    unsafe extern "Rust" {
        fn rust_main() -> !;
    }
    unsafe {
        rust_main();
    }
}

#[unsafe(no_mangle)]
extern "C" fn _exit(ec: c_int) -> ! {
    HOST.exit(ec);
}
