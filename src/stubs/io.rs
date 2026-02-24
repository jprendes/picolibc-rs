use core::ffi::*;

use crate::io::Errno;

use crate::host::{HOST, SeekFrom};

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
