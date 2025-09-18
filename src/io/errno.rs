#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Errno(u32);

impl Errno {
    pub const EPERM: Errno = Errno(picolibc_sys::EPERM);
    pub const ENOENT: Errno = Errno(picolibc_sys::ENOENT);
    pub const ESRCH: Errno = Errno(picolibc_sys::ESRCH);
    pub const EINTR: Errno = Errno(picolibc_sys::EINTR);
    pub const EIO: Errno = Errno(picolibc_sys::EIO);
    pub const ENXIO: Errno = Errno(picolibc_sys::ENXIO);
    pub const E2BIG: Errno = Errno(picolibc_sys::E2BIG);
    pub const ENOEXEC: Errno = Errno(picolibc_sys::ENOEXEC);
    pub const EBADF: Errno = Errno(picolibc_sys::EBADF);
    pub const ECHILD: Errno = Errno(picolibc_sys::ECHILD);
    pub const EAGAIN: Errno = Errno(picolibc_sys::EAGAIN);
    pub const ENOMEM: Errno = Errno(picolibc_sys::ENOMEM);
    pub const EACCES: Errno = Errno(picolibc_sys::EACCES);
    pub const EFAULT: Errno = Errno(picolibc_sys::EFAULT);
    pub const ENOTBLK: Errno = Errno(picolibc_sys::ENOTBLK);
    pub const EBUSY: Errno = Errno(picolibc_sys::EBUSY);
    pub const EEXIST: Errno = Errno(picolibc_sys::EEXIST);
    pub const EXDEV: Errno = Errno(picolibc_sys::EXDEV);
    pub const ENODEV: Errno = Errno(picolibc_sys::ENODEV);
    pub const ENOTDIR: Errno = Errno(picolibc_sys::ENOTDIR);
    pub const EISDIR: Errno = Errno(picolibc_sys::EISDIR);
    pub const EINVAL: Errno = Errno(picolibc_sys::EINVAL);
    pub const ENFILE: Errno = Errno(picolibc_sys::ENFILE);
    pub const EMFILE: Errno = Errno(picolibc_sys::EMFILE);
    pub const ENOTTY: Errno = Errno(picolibc_sys::ENOTTY);
    pub const ETXTBSY: Errno = Errno(picolibc_sys::ETXTBSY);
    pub const EFBIG: Errno = Errno(picolibc_sys::EFBIG);
    pub const ENOSPC: Errno = Errno(picolibc_sys::ENOSPC);
    pub const ESPIPE: Errno = Errno(picolibc_sys::ESPIPE);
    pub const EROFS: Errno = Errno(picolibc_sys::EROFS);
    pub const EMLINK: Errno = Errno(picolibc_sys::EMLINK);
    pub const EPIPE: Errno = Errno(picolibc_sys::EPIPE);
    pub const EDOM: Errno = Errno(picolibc_sys::EDOM);
    pub const ERANGE: Errno = Errno(picolibc_sys::ERANGE);
    pub const ENOMSG: Errno = Errno(picolibc_sys::ENOMSG);
    pub const EIDRM: Errno = Errno(picolibc_sys::EIDRM);
    pub const ECHRNG: Errno = Errno(picolibc_sys::ECHRNG);
    pub const EL2NSYNC: Errno = Errno(picolibc_sys::EL2NSYNC);
    pub const EL3HLT: Errno = Errno(picolibc_sys::EL3HLT);
    pub const EL3RST: Errno = Errno(picolibc_sys::EL3RST);
    pub const ELNRNG: Errno = Errno(picolibc_sys::ELNRNG);
    pub const EUNATCH: Errno = Errno(picolibc_sys::EUNATCH);
    pub const ENOCSI: Errno = Errno(picolibc_sys::ENOCSI);
    pub const EL2HLT: Errno = Errno(picolibc_sys::EL2HLT);
    pub const EDEADLK: Errno = Errno(picolibc_sys::EDEADLK);
    pub const ENOLCK: Errno = Errno(picolibc_sys::ENOLCK);
    pub const EBADE: Errno = Errno(picolibc_sys::EBADE);
    pub const EBADR: Errno = Errno(picolibc_sys::EBADR);
    pub const EXFULL: Errno = Errno(picolibc_sys::EXFULL);
    pub const ENOANO: Errno = Errno(picolibc_sys::ENOANO);
    pub const EBADRQC: Errno = Errno(picolibc_sys::EBADRQC);
    pub const EBADSLT: Errno = Errno(picolibc_sys::EBADSLT);
    pub const EDEADLOCK: Errno = Errno(picolibc_sys::EDEADLOCK);
    pub const EBFONT: Errno = Errno(picolibc_sys::EBFONT);
    pub const ENOSTR: Errno = Errno(picolibc_sys::ENOSTR);
    pub const ENODATA: Errno = Errno(picolibc_sys::ENODATA);
    pub const ETIME: Errno = Errno(picolibc_sys::ETIME);
    pub const ENOSR: Errno = Errno(picolibc_sys::ENOSR);
    pub const ENONET: Errno = Errno(picolibc_sys::ENONET);
    pub const ENOPKG: Errno = Errno(picolibc_sys::ENOPKG);
    pub const EREMOTE: Errno = Errno(picolibc_sys::EREMOTE);
    pub const ENOLINK: Errno = Errno(picolibc_sys::ENOLINK);
    pub const EADV: Errno = Errno(picolibc_sys::EADV);
    pub const ESRMNT: Errno = Errno(picolibc_sys::ESRMNT);
    pub const ECOMM: Errno = Errno(picolibc_sys::ECOMM);
    pub const EPROTO: Errno = Errno(picolibc_sys::EPROTO);
    pub const EMULTIHOP: Errno = Errno(picolibc_sys::EMULTIHOP);
    pub const ELBIN: Errno = Errno(picolibc_sys::ELBIN);
    pub const EDOTDOT: Errno = Errno(picolibc_sys::EDOTDOT);
    pub const EBADMSG: Errno = Errno(picolibc_sys::EBADMSG);
    pub const EFTYPE: Errno = Errno(picolibc_sys::EFTYPE);
    pub const ENOTUNIQ: Errno = Errno(picolibc_sys::ENOTUNIQ);
    pub const EBADFD: Errno = Errno(picolibc_sys::EBADFD);
    pub const EREMCHG: Errno = Errno(picolibc_sys::EREMCHG);
    pub const ELIBACC: Errno = Errno(picolibc_sys::ELIBACC);
    pub const ELIBBAD: Errno = Errno(picolibc_sys::ELIBBAD);
    pub const ELIBSCN: Errno = Errno(picolibc_sys::ELIBSCN);
    pub const ELIBMAX: Errno = Errno(picolibc_sys::ELIBMAX);
    pub const ELIBEXEC: Errno = Errno(picolibc_sys::ELIBEXEC);
    pub const ENOSYS: Errno = Errno(picolibc_sys::ENOSYS);
    pub const ENOTEMPTY: Errno = Errno(picolibc_sys::ENOTEMPTY);
    pub const ENAMETOOLONG: Errno = Errno(picolibc_sys::ENAMETOOLONG);
    pub const ELOOP: Errno = Errno(picolibc_sys::ELOOP);
    pub const EOPNOTSUPP: Errno = Errno(picolibc_sys::EOPNOTSUPP);
    pub const EPFNOSUPPORT: Errno = Errno(picolibc_sys::EPFNOSUPPORT);
    pub const ECONNRESET: Errno = Errno(picolibc_sys::ECONNRESET);
    pub const ENOBUFS: Errno = Errno(picolibc_sys::ENOBUFS);
    pub const EAFNOSUPPORT: Errno = Errno(picolibc_sys::EAFNOSUPPORT);
    pub const EPROTOTYPE: Errno = Errno(picolibc_sys::EPROTOTYPE);
    pub const ENOTSOCK: Errno = Errno(picolibc_sys::ENOTSOCK);
    pub const ENOPROTOOPT: Errno = Errno(picolibc_sys::ENOPROTOOPT);
    pub const ESHUTDOWN: Errno = Errno(picolibc_sys::ESHUTDOWN);
    pub const ECONNREFUSED: Errno = Errno(picolibc_sys::ECONNREFUSED);
    pub const EADDRINUSE: Errno = Errno(picolibc_sys::EADDRINUSE);
    pub const ECONNABORTED: Errno = Errno(picolibc_sys::ECONNABORTED);
    pub const ENETUNREACH: Errno = Errno(picolibc_sys::ENETUNREACH);
    pub const ENETDOWN: Errno = Errno(picolibc_sys::ENETDOWN);
    pub const ETIMEDOUT: Errno = Errno(picolibc_sys::ETIMEDOUT);
    pub const EHOSTDOWN: Errno = Errno(picolibc_sys::EHOSTDOWN);
    pub const EHOSTUNREACH: Errno = Errno(picolibc_sys::EHOSTUNREACH);
    pub const EINPROGRESS: Errno = Errno(picolibc_sys::EINPROGRESS);
    pub const EALREADY: Errno = Errno(picolibc_sys::EALREADY);
    pub const EDESTADDRREQ: Errno = Errno(picolibc_sys::EDESTADDRREQ);
    pub const EMSGSIZE: Errno = Errno(picolibc_sys::EMSGSIZE);
    pub const EPROTONOSUPPORT: Errno = Errno(picolibc_sys::EPROTONOSUPPORT);
    pub const ESOCKTNOSUPPORT: Errno = Errno(picolibc_sys::ESOCKTNOSUPPORT);
    pub const EADDRNOTAVAIL: Errno = Errno(picolibc_sys::EADDRNOTAVAIL);
    pub const ENETRESET: Errno = Errno(picolibc_sys::ENETRESET);
    pub const EISCONN: Errno = Errno(picolibc_sys::EISCONN);
    pub const ENOTCONN: Errno = Errno(picolibc_sys::ENOTCONN);
    pub const ETOOMANYREFS: Errno = Errno(picolibc_sys::ETOOMANYREFS);
    pub const EPROCLIM: Errno = Errno(picolibc_sys::EPROCLIM);
    pub const EUSERS: Errno = Errno(picolibc_sys::EUSERS);
    pub const EDQUOT: Errno = Errno(picolibc_sys::EDQUOT);
    pub const ESTALE: Errno = Errno(picolibc_sys::ESTALE);
    pub const ENOTSUP: Errno = Errno(picolibc_sys::ENOTSUP);
    pub const ENOMEDIUM: Errno = Errno(picolibc_sys::ENOMEDIUM);
    pub const EILSEQ: Errno = Errno(picolibc_sys::EILSEQ);
    pub const EOVERFLOW: Errno = Errno(picolibc_sys::EOVERFLOW);
    pub const ECANCELED: Errno = Errno(picolibc_sys::ECANCELED);
    pub const ENOTRECOVERABLE: Errno = Errno(picolibc_sys::ENOTRECOVERABLE);
    pub const EOWNERDEAD: Errno = Errno(picolibc_sys::EOWNERDEAD);
    pub const ESTRPIPE: Errno = Errno(picolibc_sys::ESTRPIPE);
    pub const EHWPOISON: Errno = Errno(picolibc_sys::EHWPOISON);
    pub const EISNAM: Errno = Errno(picolibc_sys::EISNAM);
    pub const EKEYEXPIRED: Errno = Errno(picolibc_sys::EKEYEXPIRED);
    pub const EKEYREJECTED: Errno = Errno(picolibc_sys::EKEYREJECTED);
    pub const EKEYREVOKED: Errno = Errno(picolibc_sys::EKEYREVOKED);
    pub const EWOULDBLOCK: Errno = Errno(picolibc_sys::EWOULDBLOCK);

    pub const fn from_raw(raw: u32) -> Self {
        Errno(raw)
    }

    pub const fn as_raw(self) -> u32 {
        self.0
    }

    pub fn set_errno(self) {
        set_errno(self.0);
    }

    pub fn from_errno() -> Self {
        Errno(get_errno())
    }
}

unsafe extern "C" {
    fn __picolibc_errno_location() -> *mut core::ffi::c_int;
}

fn set_errno(val: u32) {
    unsafe { __picolibc_errno_location().write(val as _) };
}

fn get_errno() -> u32 {
    unsafe { __picolibc_errno_location().read() as _ }
}
