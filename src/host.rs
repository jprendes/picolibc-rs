use core::ffi::*;

use crate::io::Errno;

#[cfg(feature = "linux")]
mod linux;

#[cfg(feature = "linux")]
pub use linux::LinuxHost;

mod host_imp {
    unsafe extern "Rust" {
        pub(crate) safe static __HOST: &'static dyn super::Host;
    }
}

pub(crate) use host_imp::__HOST as HOST;

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

pub type Result<T> = core::result::Result<T, Errno>;

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
    fn get_time(&self, clock: Clock) -> Result<Timespec> {
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
