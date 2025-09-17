use crate::eprintln;

pub fn exit(code: i32) -> ! {
    unsafe { picolibc_sys::exit(code) }
}

pub trait Termination {
    /// Is called to get the representation of the value as status code.
    /// This status code is returned to the operating system.
    fn report(self) -> ExitCode;
}

impl Termination for () {
    #[inline]
    fn report(self) -> ExitCode {
        ExitCode::SUCCESS
    }
}

/*
impl Termination for ! {
    fn report(self) -> ExitCode {
        self
    }
}
*/

impl Termination for core::convert::Infallible {
    fn report(self) -> ExitCode {
        match self {}
    }
}

impl Termination for ExitCode {
    #[inline]
    fn report(self) -> ExitCode {
        self
    }
}

impl<T: Termination, E: core::fmt::Debug> Termination for Result<T, E> {
    fn report(self) -> ExitCode {
        match self {
            Ok(val) => val.report(),
            Err(err) => {
                eprintln!("Error: {err:?}");
                ExitCode::FAILURE
            }
        }
    }
}

pub struct ExitCode(core::ffi::c_int);

impl ExitCode {
    /// The canonical `ExitCode` for successful termination on this platform.
    ///
    /// Note that a `()`-returning `main` implicitly results in a successful
    /// termination, so there's no need to return this from `main` unless
    /// you're also returning other possible codes.
    pub const SUCCESS: ExitCode = ExitCode(0);

    /// The canonical `ExitCode` for unsuccessful termination on this platform.
    ///
    /// If you're only returning this and `SUCCESS` from `main`, consider
    /// instead returning `Err(_)` and `Ok(())` respectively, which will
    /// return the same codes (but will also `eprintln!` the error).
    pub const FAILURE: ExitCode = ExitCode(1);

    /// Exit the current process with the given `ExitCode`.
    ///
    /// Note that this has the same caveats as [`process::exit()`][exit], namely that this function
    /// terminates the process immediately, so no destructors on the current stack or any other
    /// thread's stack will be run. Also see those docs for some important notes on interop with C
    /// code. If a clean shutdown is needed, it is recommended to simply return this ExitCode from
    /// the `main` function, as demonstrated in the [type documentation](#examples).
    ///
    /// # Differences from `process::exit()`
    ///
    /// `process::exit()` accepts any `i32` value as the exit code for the process; however, there
    /// are platforms that only use a subset of that value (see [`process::exit` platform-specific
    /// behavior][exit#platform-specific-behavior]). `ExitCode` exists because of this; only
    /// `ExitCode`s that are supported by a majority of our platforms can be created, so those
    /// problems don't exist (as much) with this method.
    ///
    /// # Examples
    ///
    /// ```
    /// #![feature(exitcode_exit_method)]
    /// # use std::process::ExitCode;
    /// # use std::fmt;
    /// # enum UhOhError { GenericProblem, Specific, WithCode { exit_code: ExitCode, _x: () } }
    /// # impl fmt::Display for UhOhError {
    /// #     fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result { unimplemented!() }
    /// # }
    /// // there's no way to gracefully recover from an UhOhError, so we just
    /// // print a message and exit
    /// fn handle_unrecoverable_error(err: UhOhError) -> ! {
    ///     eprintln!("UH OH! {err}");
    ///     let code = match err {
    ///         UhOhError::GenericProblem => ExitCode::FAILURE,
    ///         UhOhError::Specific => ExitCode::from(3),
    ///         UhOhError::WithCode { exit_code, .. } => exit_code,
    ///     };
    ///     code.exit_process()
    /// }
    /// ```
    pub fn exit_process(self) -> ! {
        unsafe { picolibc_sys::exit(self.to_i32()) }
    }
}

impl ExitCode {
    // This is private/perma-unstable because ExitCode is opaque; we don't know that i32 will serve
    // all usecases, for example windows seems to use u32, unix uses the 8-15th bits of an i32, we
    // likely want to isolate users anything that could restrict the platform specific
    // representation of an ExitCode
    //
    // More info: https://internals.rust-lang.org/t/mini-pre-rfc-redesigning-process-exitstatus/5426
    /// Converts an `ExitCode` into an i32
    #[inline]
    #[doc(hidden)]
    pub fn to_i32(self) -> i32 {
        self.0 as _
    }
}

/// The default value is [`ExitCode::SUCCESS`]
impl Default for ExitCode {
    fn default() -> Self {
        ExitCode::SUCCESS
    }
}

impl From<u8> for ExitCode {
    /// Constructs an `ExitCode` from an arbitrary u8 value.
    fn from(code: u8) -> Self {
        ExitCode(code as core::ffi::c_int)
    }
}
