//! Emulated Thread-Local Storage (TLS) implementation.
//!
//! This module provides emulated TLS support for environments where native TLS
//! is not available. For every TLS variable `xyz`, there is one `EmutlsControl`
//! variable named `__emutls_v.xyz`. If `xyz` has a non-zero initial value,
//! the control's `value` field will point to `__emutls_t.xyz` with the initial value.
//!
//! This implementation is based on LLVM's compiler-rt emutls:
//! https://github.com/llvm/llvm-project/blob/main/compiler-rt/lib/builtins/emutls.c

use alloc::rc::Rc;
use core::cell::RefCell;
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};

use hashbrown::HashMap;
use spin::{Lazy, RwLock};

use crate::host::HOST;

/// Control structure for an emulated TLS variable.
///
/// This matches the ABI expected by LLVM's `__emutls_get_address`.
#[repr(C)]
pub(crate) struct EmutlsControl<T> {
    /// Size of the TLS object in bytes.
    size: usize,
    /// Alignment of the TLS object in bytes.
    align: usize,
    /// Index into the thread-local address array (1-based), or direct address in single-thread mode.
    index: AtomicUsize,
    /// Pointer to the initial value (null if zero-initialized).
    value: Option<NonNull<u8>>,
    /// Phantom data to associate the control with the TLS variable type.
    _marker: core::marker::PhantomData<T>,
}

// The control structure is thread-safe because the only mutable state is the `index` field,
// which is an atomic that is only written once. After the index is set, the control is
// effectively immutable.
unsafe impl<T> Send for EmutlsControl<T> {}
unsafe impl<T> Sync for EmutlsControl<T> {}

impl<T> EmutlsControl<T> {
    pub(crate) const fn zeroed() -> Self {
        Self {
            size: core::mem::size_of::<T>(),
            align: core::mem::align_of::<T>(),
            index: AtomicUsize::new(0),
            value: None,
            _marker: core::marker::PhantomData,
        }
    }

    /// Entry point for emulated TLS address lookup.
    ///
    /// This function is called by the compiler-generated code to get the address
    /// of a thread-local variable. It handles lazy allocation and initialization.
    pub(crate) fn get(&self) -> &'static T {
        // Get the index for this control, allocating it if necessary.
        let index = self.get_index();

        let thread_id = HOST.thread_id().expect("TLS not supported");

        let address_array = TlsAddressArray::get_for_thread(thread_id);

        // unwrap is safe because we just ensured capacity
        let ptr = address_array
            .get_or_insert_with(index, || self.allocate_object())
            .cast();

        unsafe { ptr.as_ref() }
    }

    /// Gets the index for this control, assigning a new one if necessary.
    ///
    /// This function is thread-safe and ensures each control structure
    /// gets a unique index exactly once.
    fn get_index(&self) -> usize {
        let index = self as *const Self as usize;
        self.index.store(index, Ordering::Release);
        index
    }

    /// Allocates and initializes a new TLS object based on the control structure.
    fn allocate_object(&self) -> NonNull<u8> {
        let size = self.size;
        let align = self
            .align
            .max(core::mem::size_of::<*mut u8>())
            .next_power_of_two();

        // Avoid zero-size allocation as posix_memalign might return null.
        let total_size = size.max(1);

        let mut addr = ptr::null_mut();
        let res = unsafe { picolibc_sys::posix_memalign(&raw mut addr as _, align, total_size) };
        assert_eq!(res, 0, "allocation failed: posix_memalign failed");

        // SAFETY: posix_memalign guarantees that `addr` is valid and aligned if it returns 0.
        let addr = unsafe { NonNull::new_unchecked(addr) };

        // Initialize the object
        // SAFETY: ptr is valid and points to at least `size` bytes
        unsafe {
            match self.value {
                None => {
                    // Zero-initialize
                    ptr::write_bytes(addr.as_ptr(), 0, size);
                }
                Some(value) => {
                    // Copy initial value
                    ptr::copy_nonoverlapping(value.as_ptr(), addr.as_ptr(), size);
                }
            }
        }

        addr
    }
}

/// Array of TLS object addresses for the current thread.
#[derive(Default, Clone)]
pub(crate) struct TlsAddressArray {
    inner: Rc<RefCell<TlsAddressArrayInner>>,
}

#[derive(Default)]
struct TlsAddressArrayInner {
    address_array: HashMap<usize, NonNull<u8>>,
}

// This is safe as a particular TlsAddressArray is only accessed by its owning thread only.
unsafe impl Send for TlsAddressArray {}
unsafe impl Sync for TlsAddressArray {}

impl TlsAddressArray {
    fn get_for_thread(thread_id: usize) -> Self {
        static TLS_ADDRESS_ARRAY: Lazy<RwLock<HashMap<usize, TlsAddressArray>>> =
            Lazy::new(Default::default);

        let guard = TLS_ADDRESS_ARRAY.read();
        if let Some(array) = guard.get(&thread_id).cloned() {
            return array;
        }
        drop(guard); // Release read lock before acquiring write lock
        let mut guard = TLS_ADDRESS_ARRAY.write();
        guard.entry(thread_id).or_default().clone()
    }

    fn get_or_insert_with(&self, key: usize, or_else: impl FnOnce() -> NonNull<u8>) -> NonNull<u8> {
        self.inner.borrow_mut().get_or_insert_with(key, or_else)
    }
}

impl TlsAddressArrayInner {
    fn get_or_insert_with(
        &mut self,
        key: usize,
        or_else: impl FnOnce() -> NonNull<u8>,
    ) -> NonNull<u8> {
        *self.address_array.entry(key).or_insert_with(or_else)
    }
}

impl Drop for TlsAddressArrayInner {
    fn drop(&mut self) {
        // Free all allocated TLS objects.
        // Note: We don't know the original layout, so we can't properly deallocate.
        // In practice, this is only called at program exit in single-threaded mode.
        for addr in self.address_array.values() {
            unsafe { picolibc_sys::free(addr.as_ptr() as _) };
        }
    }
}
