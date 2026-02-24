use core::ffi::*;

use crate::io::Errno;

use crate::host::{Clock, HOST};

#[unsafe(no_mangle)]
extern "C" fn clock_gettime(
    clock: picolibc_sys::clockid_t,
    ts: *mut picolibc_sys::timespec,
) -> c_int {
    #![allow(clippy::not_unsafe_ptr_arg_deref)]

    let Some(ts) = (unsafe { ts.as_mut() }) else {
        return 0;
    };

    let clock = match clock as u32 {
        picolibc_sys::CLOCK_REALTIME => Clock::Realtime,
        picolibc_sys::CLOCK_MONOTONIC => Clock::Monotonic,
        _ => {
            Errno::EINVAL.set_errno();
            return -1;
        }
    };

    match HOST.get_time(clock) {
        Ok(t) => {
            ts.tv_sec = t.sec;
            ts.tv_nsec = t.nsec as _;
            0
        }
        Err(err) => {
            err.set_errno();
            -1
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn gettimeofday(tv: *mut picolibc_sys::timeval, _tz: *mut c_void) -> c_int {
    #![allow(clippy::not_unsafe_ptr_arg_deref)]

    let Some(tv) = (unsafe { tv.as_mut() }) else {
        return 0;
    };

    let mut ts = picolibc_sys::timespec::default();

    if clock_gettime(picolibc_sys::CLOCK_REALTIME as _, &raw mut ts) != 0 {
        // errno is already set by clock_gettime, so we just need to return -1.
        return -1;
    }

    tv.tv_sec = ts.tv_sec;
    tv.tv_usec = (ts.tv_nsec / 1000) as _;
    0
}
