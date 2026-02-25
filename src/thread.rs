//! A thread_local implementation

use core::cell::{Cell, RefCell, UnsafeCell};

use crate::emutls::EmutlsControl;

pub struct LocalKey<T: 'static> {
    init: fn() -> T,
    inner: EmutlsControl<State<T>>,
}

pub struct AccessError;

/// This enum is such that when its byte representation is all zeroes,
/// it must be the Uninit variant.
/// When we create a EmutlsControl with no initial value, its initial
/// byte representation will be all zeroes, so it will start in the
/// Uninit state.
#[repr(u8)]
enum StateInner<T> {
    #[allow(dead_code)]
    // rustc believes this variant is never created, but it is created when the EmutlsControl is zero-initialized.
    Uninit = 0,
    Init(T),
}

impl<T> StateInner<T> {
    fn get_or_init(&mut self, init: impl FnOnce() -> T) -> &T {
        if let StateInner::Uninit = self {
            *self = StateInner::Init(init());
        }
        if let StateInner::Init(value) = self {
            value
        } else {
            unreachable!()
        }
    }
}

#[repr(transparent)]
struct State<T> {
    inner: UnsafeCell<StateInner<T>>,
}

impl<T> State<T> {
    fn get_or_init(&self, init: impl FnOnce() -> T) -> &T {
        let inner = unsafe { &mut *self.inner.get() };
        inner.get_or_init(init)
    }
}

impl<T: 'static> LocalKey<T> {
    #[doc(hidden)]
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            init,
            inner: EmutlsControl::zeroed(),
        }
    }

    /// Accesses the thread-local value, initializing it if necessary.
    /// The provided closure is called with a reference to the value.
    /// This is the main entry point for accessing thread-local variables.
    pub fn with<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        unsafe extern "C" {
            fn __emutls_get_address(control: *const EmutlsControl<u8>) -> *const State<u8>;
        }
        let state = unsafe {
            __emutls_get_address(&raw const self.inner as _)
                .cast::<State<T>>()
                .as_ref()
                .unwrap()
        };
        let value_ref = state.get_or_init(&self.init);
        f(value_ref)
    }

    /// Accesses the thread-local value, initializing it if necessary.
    /// The provided closure is called with a reference to the value.
    /// Returns Err(AccessError) if the key has been destroyed for this thread.
    pub fn try_with<R>(&'static self, f: impl FnOnce(&T) -> R) -> Result<R, AccessError> {
        // We don't destroy keys yet, so this is the same as with() for now.
        // In the future, we may want run drop for keys at thread exit and return an error if the key has been destroyed for this thread.
        Ok(self.with(f))
    }
}

impl<T: 'static> LocalKey<Cell<T>> {
    pub fn set(&'static self, value: T) {
        self.with(|cell| cell.set(value));
    }

    pub fn get(&'static self) -> T
    where
        T: Copy,
    {
        self.with(|cell| cell.get())
    }

    pub fn take(&'static self) -> T
    where
        T: Default,
    {
        self.with(|cell| cell.take())
    }

    pub fn replace(&'static self, value: T) -> T {
        self.with(|cell| cell.replace(value))
    }

    pub fn update(&'static self, f: impl FnOnce(T) -> T)
    where
        T: Copy,
    {
        self.with(|cell| cell.update(f))
    }
}

impl<T: 'static> LocalKey<RefCell<T>> {
    pub fn with_borrow<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        self.with(|cell| f(&*cell.borrow()))
    }

    pub fn with_borrow_mut<R>(&'static self, f: impl FnOnce(&mut T) -> R) -> R {
        self.with(|cell| f(&mut *cell.borrow_mut()))
    }
}

#[macro_export]
macro_rules! thread_local {
    () => {};
    ($(#[$attr:meta])* $vis:vis static $name:ident: $ty:ty = const $init:expr; $($tt:tt)*) => {
        $(#[$attr])*
        $vis static $name: $crate::thread::LocalKey<$ty> = $crate::thread::LocalKey::new(const || $init);
        $crate::thread_local!{ $($tt)* }
    };
    ($(#[$attr:meta])* $vis:vis static $name:ident: $ty:ty = $init:expr; $($tt:tt)*) => {
        $(#[$attr])*
        $vis static $name: $crate::thread::LocalKey<$ty> = $crate::thread::LocalKey::new(|| $init);
        $crate::thread_local!{ $($tt)* }
    };
}
