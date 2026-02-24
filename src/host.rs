use core::ffi::*;
use core::ptr::{NonNull, null_mut};

use crate::emutls::EmutlsControl;
use crate::io::Errno;

#[cfg(feature = "linux")]
mod linux;

#[cfg(feature = "linux")]
pub use linux::LinuxHost;

unsafe extern "Rust" {
    fn rust_main() -> !;
    #[unsafe(no_mangle)]
    pub(crate) safe static __HOST: &'static dyn Host;
}

pub(crate) use __HOST as HOST;

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

pub trait Host: Sync {
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
        #[allow(clippy::empty_loop)]
        loop {}
    }
    fn mmap(&self, addr: *mut u8, size: usize) -> Result<&mut [u8]> {
        Err(Errno::ENOMEM)
    }
    fn thread_id(&self) -> Result<usize> {
        Ok(1)
    }
}

#[unsafe(no_mangle)]
extern "C" fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize {
    let buf = unsafe { core::slice::from_raw_parts_mut(buf as *mut u8, count) };

    match HOST.read(fd, buf) {
        Ok(b) => b as isize,
        Err(err) => {
            err.set_errno();
            -1
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn write(fd: c_int, buf: *const c_void, count: usize) -> isize {
    let buf = unsafe { core::slice::from_raw_parts(buf as *const u8, count) };

    match HOST.write(fd, buf) {
        Ok(b) => b as isize,
        Err(err) => {
            err.set_errno();
            -1
        }
    }
}

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

    match HOST.gettime(clock) {
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
extern "C" fn _exit(ec: c_int) -> ! {
    HOST.exit(ec);
}

#[unsafe(no_mangle)]
extern "C" fn lseek(fd: c_int, offset: c_long, whence: c_int) -> c_long {
    let offset = match whence as u32 {
        picolibc_sys::SEEK_SET => SeekFrom::Start(offset as u64),
        picolibc_sys::SEEK_CUR => SeekFrom::Current(offset),
        picolibc_sys::SEEK_END => SeekFrom::End(offset),
        _ => {
            Errno::EINVAL.set_errno();
            return -1;
        }
    };

    match HOST.lseek(fd, offset) {
        Ok(n) => n as c_long,
        Err(err) => {
            err.set_errno();
            -1
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn close(fd: c_int) -> c_int {
    match HOST.close(fd) {
        Ok(()) => 0,
        Err(err) => {
            err.set_errno();
            -1
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn _mmap(addr: *mut u8, size: usize) -> *mut u8 {
    match HOST.mmap(addr, size) {
        Ok(ptr) => ptr.as_mut_ptr(),
        Err(err) => {
            err.set_errno();
            null_mut()
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn rust_start() -> ! {
    unsafe {
        rust_main();
    }
}

#[unsafe(no_mangle)]
extern "C" fn __tls_get_address(control: &EmutlsControl<u8>) -> NonNull<u8> {
    control.get().into()
}
