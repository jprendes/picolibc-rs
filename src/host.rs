use core::ffi::*;
use core::ptr::null_mut;

use crate::io::Errno;

mod linux;

pub use linux::LinuxHost;

pub fn get_host() -> &'static dyn Host {
    // This is not thread-safe, but it's ok because
    // we currently don't support threads
    static mut HOST: Option<&'static dyn Host> = None;

    unsafe extern "Rust" {
        fn _get_host() -> &'static dyn Host;
    }

    *unsafe { HOST }.get_or_insert_with(|| unsafe { _get_host() })
}

pub enum SeekFrom {
    Start(u64),
    End(i64),
    Current(i64),
}

pub struct Timespec {
    pub sec: i64,
    pub nsec: u32,
}

pub enum Clock {
    Realtime,
    Monotonic,
}

type Result<T> = core::result::Result<T, Errno>;

pub trait Host {
    #![allow(unused_variables)]

    fn read(&self, fd: c_int, buf: &mut [u8]) -> Result<usize> {
        Ok(0)
    }
    fn write(&self, fd: c_int, buf: &[u8]) -> Result<usize> {
        Ok(buf.len())
    }
    fn lseek(&self, fd: c_int, offset: SeekFrom) -> Result<usize> {
        Ok(0)
    }
    fn close(&self, fd: c_int) -> Result<()> {
        Ok(())
    }
    fn gettime(&self, clock: Clock) -> Result<Timespec> {
        Ok(Timespec { sec: 0, nsec: 0 })
    }
    fn exit(&self, ec: c_int) -> ! {
        loop {}
    }
    fn brk(&self, addr: *const ()) -> Result<*mut ()> {
        Ok(null_mut())
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize {
    let buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, count) };

    match get_host().read(fd, buf) {
        Ok(b) => b as isize,
        Err(err) => {
            err.set_errno();
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn write(fd: c_int, buf: *const c_void, count: usize) -> isize {
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, count) };

    match get_host().write(fd, buf) {
        Ok(b) => b as isize,
        Err(err) => {
            err.set_errno();
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn clock_gettime(
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

    match get_host().gettime(clock) {
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
pub extern "C" fn _exit(ec: c_int) -> ! {
    get_host().exit(ec);
}

#[unsafe(no_mangle)]
pub extern "C" fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long {
    let offset = match whence as u32 {
        picolibc_sys::SEEK_SET => SeekFrom::Start(offset as u64),
        picolibc_sys::SEEK_CUR => SeekFrom::Current(offset),
        picolibc_sys::SEEK_END => SeekFrom::End(offset),
        _ => {
            Errno::EINVAL.set_errno();
            return -1;
        }
    };

    match get_host().lseek(fd, offset) {
        Ok(n) => n as c_long,
        Err(err) => {
            err.set_errno();
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn close(fd: c_int) -> c_int {
    match get_host().close(fd) {
        Ok(()) => 0,
        Err(err) => {
            err.set_errno();
            -1
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _brk(addr: *const ()) -> *mut () {
    match get_host().brk(addr) {
        Ok(a) => a,
        Err(err) => {
            err.set_errno();
            null_mut()
        }
    }
}
