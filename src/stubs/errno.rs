use core::ffi::c_int;

use crate::thread_local;

thread_local! {
    static ERRNO: c_int = 0;
}

#[unsafe(no_mangle)]
extern "C" fn __errno_location() -> *mut c_int {
    ERRNO.with(|errno| errno as *const _ as *mut _)
}
