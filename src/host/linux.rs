use core::ffi::*;

use linux_syscall::{SYS_brk, SYS_clock_gettime, SYS_exit, SYS_exit_group, SYS_read, SYS_write};

use crate::io::Errno;

use super::{Clock, Host, Result, Timespec};
pub struct LinuxHost;

macro_rules! syscall {
    ($($arg:tt)*) => {
        core::convert::TryInto::<usize>::try_into(unsafe { linux_syscall::syscall!($($arg)*) })
            .map_err(Errno::from)
    };
}

impl Host for LinuxHost {
    fn read(&self, fd: c_int, buf: &mut [u8]) -> Result<usize> {
        // Only stdin (fd=0)
        if fd != 0 {
            return Ok(0);
        }

        syscall!(SYS_read, fd, buf.as_mut_ptr(), buf.len())
    }

    fn write(&self, fd: c_int, buf: &[u8]) -> Result<usize> {
        // Only stdout (fd=1) and stderr (fd=2)
        if fd != 1 && fd != 2 {
            return Ok(buf.len());
        }

        syscall!(SYS_write, fd, buf.as_ptr(), buf.len())
    }

    fn gettime(&self, clock: Clock) -> Result<Timespec> {
        const CLOCK_REALTIME: c_ulong = 0;
        const CLOCK_MONOTONIC: c_ulong = 1;

        let clock_id: c_ulong = match clock {
            Clock::Realtime => CLOCK_REALTIME,
            Clock::Monotonic => CLOCK_MONOTONIC,
        };

        let mut ts = [0i64; 2];
        syscall!(SYS_clock_gettime, clock_id, ts.as_mut_ptr())?;

        Ok(Timespec {
            sec: ts[0],
            nsec: ts[1].clamp(0, 999_999_999) as _,
        })
    }

    fn exit(&self, ec: c_int) -> ! {
        let _ = syscall!(SYS_exit_group, ec);
        loop {
            let _ = syscall!(SYS_exit, ec);
        }
    }

    fn brk(&self, addr: *const ()) -> Result<*mut ()> {
        let addr = syscall!(SYS_brk, addr)?;
        Ok(addr as _)
    }
}

impl From<linux_errno::Error> for Errno {
    fn from(err: linux_errno::Error) -> Self {
        match err {
            linux_errno::EPERM => Errno::EPERM,
            linux_errno::ENOENT => Errno::ENOENT,
            linux_errno::ESRCH => Errno::ESRCH,
            linux_errno::EINTR => Errno::EINTR,
            linux_errno::EIO => Errno::EIO,
            linux_errno::ENXIO => Errno::ENXIO,
            linux_errno::E2BIG => Errno::E2BIG,
            linux_errno::ENOEXEC => Errno::ENOEXEC,
            linux_errno::EBADF => Errno::EBADF,
            linux_errno::ECHILD => Errno::ECHILD,
            linux_errno::EAGAIN => Errno::EAGAIN,
            linux_errno::ENOMEM => Errno::ENOMEM,
            linux_errno::EACCES => Errno::EACCES,
            linux_errno::EFAULT => Errno::EFAULT,
            linux_errno::ENOTBLK => Errno::ENOTBLK,
            linux_errno::EBUSY => Errno::EBUSY,
            linux_errno::EEXIST => Errno::EEXIST,
            linux_errno::EXDEV => Errno::EXDEV,
            linux_errno::ENODEV => Errno::ENODEV,
            linux_errno::ENOTDIR => Errno::ENOTDIR,
            linux_errno::EISDIR => Errno::EISDIR,
            linux_errno::EINVAL => Errno::EINVAL,
            linux_errno::ENFILE => Errno::ENFILE,
            linux_errno::EMFILE => Errno::EMFILE,
            linux_errno::ENOTTY => Errno::ENOTTY,
            linux_errno::ETXTBSY => Errno::ETXTBSY,
            linux_errno::EFBIG => Errno::EFBIG,
            linux_errno::ENOSPC => Errno::ENOSPC,
            linux_errno::ESPIPE => Errno::ESPIPE,
            linux_errno::EROFS => Errno::EROFS,
            linux_errno::EMLINK => Errno::EMLINK,
            linux_errno::EPIPE => Errno::EPIPE,
            linux_errno::EDOM => Errno::EDOM,
            linux_errno::ERANGE => Errno::ERANGE,
            linux_errno::ENOMSG => Errno::ENOMSG,
            linux_errno::EIDRM => Errno::EIDRM,
            linux_errno::ECHRNG => Errno::ECHRNG,
            linux_errno::EL2NSYNC => Errno::EL2NSYNC,
            linux_errno::EL3HLT => Errno::EL3HLT,
            linux_errno::EL3RST => Errno::EL3RST,
            linux_errno::ELNRNG => Errno::ELNRNG,
            linux_errno::EUNATCH => Errno::EUNATCH,
            linux_errno::ENOCSI => Errno::ENOCSI,
            linux_errno::EL2HLT => Errno::EL2HLT,
            linux_errno::EDEADLK => Errno::EDEADLK,
            linux_errno::ENOLCK => Errno::ENOLCK,
            linux_errno::EBADE => Errno::EBADE,
            linux_errno::EBADR => Errno::EBADR,
            linux_errno::EXFULL => Errno::EXFULL,
            linux_errno::ENOANO => Errno::ENOANO,
            linux_errno::EBADRQC => Errno::EBADRQC,
            linux_errno::EBADSLT => Errno::EBADSLT,
            //linux_errno::EDEADLOCK => Errno::EDEADLOCK,
            linux_errno::EBFONT => Errno::EBFONT,
            linux_errno::ENOSTR => Errno::ENOSTR,
            linux_errno::ENODATA => Errno::ENODATA,
            linux_errno::ETIME => Errno::ETIME,
            linux_errno::ENOSR => Errno::ENOSR,
            linux_errno::ENONET => Errno::ENONET,
            linux_errno::ENOPKG => Errno::ENOPKG,
            linux_errno::EREMOTE => Errno::EREMOTE,
            linux_errno::ENOLINK => Errno::ENOLINK,
            linux_errno::EADV => Errno::EADV,
            linux_errno::ESRMNT => Errno::ESRMNT,
            linux_errno::ECOMM => Errno::ECOMM,
            linux_errno::EPROTO => Errno::EPROTO,
            linux_errno::EMULTIHOP => Errno::EMULTIHOP,
            //linux_errno::ELBIN => Errno::ELBIN,
            linux_errno::EDOTDOT => Errno::EDOTDOT,
            linux_errno::EBADMSG => Errno::EBADMSG,
            //linux_errno::EFTYPE => Errno::EFTYPE,
            linux_errno::ENOTUNIQ => Errno::ENOTUNIQ,
            linux_errno::EBADFD => Errno::EBADFD,
            linux_errno::EREMCHG => Errno::EREMCHG,
            linux_errno::ELIBACC => Errno::ELIBACC,
            linux_errno::ELIBBAD => Errno::ELIBBAD,
            linux_errno::ELIBSCN => Errno::ELIBSCN,
            linux_errno::ELIBMAX => Errno::ELIBMAX,
            linux_errno::ELIBEXEC => Errno::ELIBEXEC,
            linux_errno::ENOSYS => Errno::ENOSYS,
            linux_errno::ENOTEMPTY => Errno::ENOTEMPTY,
            linux_errno::ENAMETOOLONG => Errno::ENAMETOOLONG,
            linux_errno::ELOOP => Errno::ELOOP,
            linux_errno::EOPNOTSUPP => Errno::EOPNOTSUPP,
            linux_errno::EPFNOSUPPORT => Errno::EPFNOSUPPORT,
            linux_errno::ECONNRESET => Errno::ECONNRESET,
            linux_errno::ENOBUFS => Errno::ENOBUFS,
            linux_errno::EAFNOSUPPORT => Errno::EAFNOSUPPORT,
            linux_errno::EPROTOTYPE => Errno::EPROTOTYPE,
            linux_errno::ENOTSOCK => Errno::ENOTSOCK,
            linux_errno::ENOPROTOOPT => Errno::ENOPROTOOPT,
            linux_errno::ESHUTDOWN => Errno::ESHUTDOWN,
            linux_errno::ECONNREFUSED => Errno::ECONNREFUSED,
            linux_errno::EADDRINUSE => Errno::EADDRINUSE,
            linux_errno::ECONNABORTED => Errno::ECONNABORTED,
            linux_errno::ENETUNREACH => Errno::ENETUNREACH,
            linux_errno::ENETDOWN => Errno::ENETDOWN,
            linux_errno::ETIMEDOUT => Errno::ETIMEDOUT,
            linux_errno::EHOSTDOWN => Errno::EHOSTDOWN,
            linux_errno::EHOSTUNREACH => Errno::EHOSTUNREACH,
            linux_errno::EINPROGRESS => Errno::EINPROGRESS,
            linux_errno::EALREADY => Errno::EALREADY,
            linux_errno::EDESTADDRREQ => Errno::EDESTADDRREQ,
            linux_errno::EMSGSIZE => Errno::EMSGSIZE,
            linux_errno::EPROTONOSUPPORT => Errno::EPROTONOSUPPORT,
            linux_errno::ESOCKTNOSUPPORT => Errno::ESOCKTNOSUPPORT,
            linux_errno::EADDRNOTAVAIL => Errno::EADDRNOTAVAIL,
            linux_errno::ENETRESET => Errno::ENETRESET,
            linux_errno::EISCONN => Errno::EISCONN,
            linux_errno::ENOTCONN => Errno::ENOTCONN,
            linux_errno::ETOOMANYREFS => Errno::ETOOMANYREFS,
            //linux_errno::EPROCLIM => Errno::EPROCLIM,
            linux_errno::EUSERS => Errno::EUSERS,
            linux_errno::EDQUOT => Errno::EDQUOT,
            linux_errno::ESTALE => Errno::ESTALE,
            //linux_errno::ENOTSUP => Errno::ENOTSUP,
            linux_errno::ENOMEDIUM => Errno::ENOMEDIUM,
            linux_errno::EILSEQ => Errno::EILSEQ,
            linux_errno::EOVERFLOW => Errno::EOVERFLOW,
            linux_errno::ECANCELED => Errno::ECANCELED,
            linux_errno::ENOTRECOVERABLE => Errno::ENOTRECOVERABLE,
            linux_errno::EOWNERDEAD => Errno::EOWNERDEAD,
            linux_errno::ESTRPIPE => Errno::ESTRPIPE,
            linux_errno::EHWPOISON => Errno::EHWPOISON,
            linux_errno::EISNAM => Errno::EISNAM,
            linux_errno::EKEYEXPIRED => Errno::EKEYEXPIRED,
            linux_errno::EKEYREJECTED => Errno::EKEYREJECTED,
            linux_errno::EKEYREVOKED => Errno::EKEYREVOKED,
            //linux_errno::EWOULDBLOCK => Errno::EWOULDBLOCK,
            err => Errno::from_raw(err.get() as u32 + 0x1_0000),
        }
    }
}
