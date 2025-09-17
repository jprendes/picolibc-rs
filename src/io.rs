use core::ffi::c_int;

#[inline]
fn write_bytes(fd: c_int, s: impl AsRef<[u8]>) -> core::fmt::Result {
    let mut msg = s.as_ref();
    while !msg.is_empty() {
        match unsafe { picolibc_sys::write(fd, msg.as_ptr() as _, msg.len()) } {
            ..=0 => break,
            n => msg = &msg[(n as usize)..],
        }
    }
    Ok(())
}

pub struct Stdout;

impl ::core::fmt::Write for Stdout {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write_bytes(1, s)
    }
}

pub struct Stderr;

impl ::core::fmt::Write for Stderr {
    #[inline]
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write_bytes(2, s)
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        let mut writer = $crate::io::Stdout;
        let _ = ::core::fmt::Write::write_fmt(&mut writer, ::core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => {{
        let mut writer = $crate::io::Stdout;
        let _ = ::core::fmt::Write::write_str(&mut writer, "\n");
    }};
    ($($arg:tt)*) => {{
        let mut writer = $crate::io::Stdout;
        let _ = ::core::fmt::Write::write_fmt(&mut writer, ::core::format_args!($($arg)*));
        let _ = ::core::fmt::Write::write_str(&mut writer, "\n");
    }};
}

#[macro_export]
macro_rules! print_str {
    ($arg:expr) => {{
        let mut writer = $crate::io::Stdout;
        let _ = ::core::fmt::Write::write_str(&mut writer, $arg);
    }};
}

#[macro_export]
macro_rules! println_str {
    ($arg:expr) => {{
        let mut writer = $crate::io::Stdout;
        let _ = ::core::fmt::Write::write_str(&mut writer, $arg);
        let _ = ::core::fmt::Write::write_str(&mut writer, "\n");
    }};
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => {{
        let mut writer = $crate::io::Stderr;
        let _ = ::core::fmt::Write::write_fmt(&mut writer, ::core::format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! eprintln {
    () => {{
        let mut writer = $crate::io::Stderr;
        let _ = ::core::fmt::Write::write_str(&mut writer, "\n");
    }};
    ($($arg:tt)*) => {{
        let mut writer = $crate::io::Stderr;
        let _ = ::core::fmt::Write::write_fmt(&mut writer, ::core::format_args!($($arg)*));
        let _ = ::core::fmt::Write::write_str(&mut writer, "\n");
    }};
}

#[macro_export]
macro_rules! eprint_str {
    ($arg:expr) => {{
        let mut writer = $crate::io::Stderr;
        let _ = ::core::fmt::Write::write_str(&mut writer, $arg);
    }};
}

#[macro_export]
macro_rules! eprintln_str {
    ($arg:expr) => {{
        let mut writer = $crate::io::Stderr;
        let _ = ::core::fmt::Write::write_str(&mut writer, $arg);
        let _ = ::core::fmt::Write::write_str(&mut writer, "\n");
    }};
}

#[macro_export]
macro_rules! dbg {
    () => {
        let mut writer = $crate::io::Stderr;
        $crate::eprintln!("[{}:{}]", $file!(), $line!())
    };
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                $crate::eprintln!("[{}:{}] {} = {:#?}", file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
