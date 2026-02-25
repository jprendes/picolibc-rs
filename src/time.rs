use core::cmp::Ordering;
use core::fmt::{Display, Formatter};
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SystemTime {
    inner: picolibc_sys::timespec,
}

pub struct SystemTimeError {
    duration: Duration,
}

impl SystemTime {
    pub const UNIX_EPOCH: Self = Self {
        inner: picolibc_sys::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    };

    pub fn now() -> Self {
        let mut timespec = picolibc_sys::timespec::default();
        unsafe {
            picolibc_sys::clock_gettime(picolibc_sys::CLOCK_REALTIME as _, &raw mut timespec);
        }
        Self { inner: timespec }
    }

    pub fn duration_since(&self, earlier: Self) -> Result<Duration, SystemTimeError> {
        if self.inner.tv_sec < earlier.inner.tv_sec
            || (self.inner.tv_sec == earlier.inner.tv_sec
                && self.inner.tv_nsec < earlier.inner.tv_nsec)
        {
            let Ok(duration) = earlier.duration_since(*self) else {
                unreachable!(
                    "duration_since only fails if self is earlier than the argument, but we just checked that it isn't"
                );
            };
            return Err(SystemTimeError { duration });
        }

        let mut tv_sec = self.inner.tv_sec - earlier.inner.tv_sec;
        let mut tv_nsec = self.inner.tv_nsec - earlier.inner.tv_nsec;

        while tv_nsec < 0 {
            // If the nanosecond difference is negative, we need to borrow one second
            tv_sec -= 1;
            tv_nsec += 1_000_000_000; // add one second in nanoseconds
        }

        Ok(Duration::new(tv_sec as u64, tv_nsec as u32))
    }

    pub fn elapsed(&self) -> Result<Duration, SystemTimeError> {
        Self::now().duration_since(*self)
    }

    pub fn checked_add(&self, duration: Duration) -> Option<Self> {
        let sec = duration.as_secs();
        let nsec = duration.subsec_nanos();

        let tv_sec = self.inner.tv_sec.checked_add_unsigned(sec)?;
        let tv_nsec = self.inner.tv_nsec.checked_add_unsigned(nsec as _)?;

        let result = Self {
            inner: picolibc_sys::timespec { tv_sec, tv_nsec },
        };

        result.normalize()
    }

    pub fn checked_sub(&self, duration: Duration) -> Option<Self> {
        let sec = duration.as_secs();
        let nsec = duration.subsec_nanos();

        let tv_sec = self.inner.tv_sec.checked_sub_unsigned(sec)?;
        let tv_nsec = self.inner.tv_nsec.checked_sub_unsigned(nsec as _)?;

        let result = Self {
            inner: picolibc_sys::timespec { tv_sec, tv_nsec },
        };

        result.normalize()
    }

    fn normalize(&self) -> Option<Self> {
        let picolibc_sys::timespec {
            mut tv_sec,
            mut tv_nsec,
        } = self.inner;

        // Handle nanosecond overflow
        while tv_nsec >= 1_000_000_000 {
            tv_sec = tv_sec.checked_add(1)?;
            tv_nsec -= 1_000_000_000;
        }

        while tv_nsec < 0 {
            tv_sec = tv_sec.checked_sub(1)?;
            tv_nsec += 1_000_000_000;
        }

        let inner = picolibc_sys::timespec { tv_sec, tv_nsec };
        Some(Self { inner })
    }
}

impl Add<Duration> for SystemTime {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding duration to SystemTime")
    }
}

impl AddAssign<Duration> for SystemTime {
    fn add_assign(&mut self, rhs: Duration) {
        *self = self
            .checked_add(rhs)
            .expect("overflow when adding duration to SystemTime");
    }
}

impl Sub<Duration> for SystemTime {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self::Output {
        self.checked_sub(rhs)
            .expect("overflow when subtracting duration from SystemTime")
    }
}

impl SubAssign<Duration> for SystemTime {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = self
            .checked_sub(rhs)
            .expect("overflow when subtracting duration from SystemTime");
    }
}

impl PartialOrd for SystemTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.inner.tv_sec < other.inner.tv_sec {
            return Some(Ordering::Less);
        }
        if self.inner.tv_sec > other.inner.tv_sec {
            return Some(Ordering::Greater);
        }
        if self.inner.tv_nsec < other.inner.tv_nsec {
            return Some(Ordering::Less);
        }
        if self.inner.tv_nsec > other.inner.tv_nsec {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Equal)
    }
}

impl Ord for SystemTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl SystemTimeError {
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

impl Display for SystemTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut buffer = [0u8; 64];
        // use the representation provided by ctime_r, which is something like "Wed Jun 30 21:49:08 1993\n\0"
        let res = unsafe { picolibc_sys::ctime_r(&self.inner.tv_sec, buffer.as_mut_ptr() as _) };
        if res.is_null() {
            return write!(f, "Invalid time");
        }
        let c_str = unsafe { core::ffi::CStr::from_bytes_until_nul(&buffer) }.unwrap();
        let str_slice = c_str.to_str().unwrap_or("Invalid time");
        let str_slice = str_slice.trim_end(); // remove the trailing newline
        write!(f, "{}", str_slice)
    }
}
