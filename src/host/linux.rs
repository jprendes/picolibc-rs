use core::ffi::*;

use linux_syscall::{
    Result as _, ResultPtr as _, ResultSize as _, SYS_brk, SYS_clock_gettime, SYS_exit,
    SYS_exit_group, SYS_read, SYS_write, syscall,
};

use super::{Clock, Host, Result, SeekFrom, Timespec};
pub struct LinuxHost;

impl Host for LinuxHost {
    fn read(&self, fd: c_int, buf: &mut [u8]) -> Result<usize> {
        // Only stdin (fd=0)
        if fd != 0 {
            return Ok(0);
        }

        unsafe { syscall!(SYS_read, fd, buf.as_mut_ptr(), buf.len()) }
            .try_usize()
            .map_err(into_error)
    }

    fn write(&self, fd: c_int, buf: &[u8]) -> Result<usize> {
        // Only stdout (fd=1) and stderr (fd=2)
        if fd != 1 && fd != 2 {
            return Ok(buf.len());
        }

        unsafe { syscall!(SYS_write, fd, buf.as_ptr(), buf.len()) }
            .try_usize()
            .map_err(into_error)
    }

    fn lseek(&self, _fd: c_int, _offset: SeekFrom) -> Result<usize> {
        Ok(0)
    }

    fn close(&self, _fd: c_int) -> Result<()> {
        Ok(())
    }

    fn gettime(&self, clock: Clock) -> Result<Timespec> {
        const CLOCK_REALTIME: c_ulong = 0;
        const CLOCK_MONOTONIC: c_ulong = 1;

        let clock_id: c_ulong = match clock {
            Clock::Realtime => CLOCK_REALTIME,
            Clock::Monotonic => CLOCK_MONOTONIC,
        };

        let mut ts = [0i64; 2];
        unsafe { syscall!(SYS_clock_gettime, clock_id, ts.as_mut_ptr()) }
            .check()
            .map_err(into_error)?;

        Ok(Timespec {
            sec: ts[0],
            nsec: ts[1].clamp(0, 999_999_999) as _,
        })
    }

    fn exit(&self, ec: c_int) -> ! {
        let _ = unsafe { syscall!(SYS_exit_group, ec) };
        loop {
            let _ = unsafe { syscall!(SYS_exit, ec) };
        }
    }

    fn brk(&self, addr: *const ()) -> Result<*mut ()> {
        unsafe { syscall!(SYS_brk, addr) }
            .try_ptr_mut()
            .map_err(into_error)
    }
}

fn into_error(err: linux_errno::Error) -> anyhow::Error {
    anyhow::format_err!("Linux error: {}", err.get())
}
