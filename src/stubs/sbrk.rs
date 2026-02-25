use core::ffi::c_void;

use crate::host::{Result, HOST};
use crate::io::Errno;

const KIB: usize = 1024;
const MIB: usize = KIB * KIB;
const GIB: usize = KIB * MIB;
const TIB: usize = KIB * GIB;

#[allow(clippy::identity_op)] // 1 * TIB looks nicer than TIB
const HEAP_START: usize = 1 * TIB; // start heap at 1 TiB
const GRANULARITY: usize = 64 * KIB; // align brk to 64 KiB
const MAX_MMAP: usize = 16 * MIB; // maximum size of a single mmap request

// Alignment constants from nano-malloc.h
const MALLOC_CHUNK_ALIGN: usize = core::mem::align_of::<MaxAlign>();
const MALLOC_HEAD: usize = core::mem::size_of::<usize>();

#[repr(C)]
union MaxAlign {
    _p: *mut c_void,
    _d: f64,
    _ll: i64,
    _s: usize,
}

struct Brk {
    brk: usize,
    brk_start: usize,
    brk_end: usize,
    min_increment: usize,
}

static BRK: spin::Mutex<Brk> = spin::Mutex::new(Brk {
    brk: HEAP_START,
    brk_start: HEAP_START,
    brk_end: HEAP_START,
    min_increment: GRANULARITY,
});

const fn align_up(val: usize, align: usize) -> usize {
    (val + align - 1) & !(align - 1)
}

impl Brk {
    fn has_capacity(&mut self, incr: usize) -> bool {
        self.brk + incr <= self.brk_end
    }

    fn ensure_capacity(&mut self, incr: usize) -> Result<()> {
        if self.has_capacity(incr) {
            return Ok(());
        }
        self.extend(incr)
    }

    fn extend_impl(&mut self, incr: usize) -> Result<()> {
        let incr = incr.max(self.min_increment);

        let new_mem = HOST.mmap(self.brk_end as *mut u8, incr)?.as_mut_ptr() as usize;

        // We managed to allocate new memory, increment the min_increment for next time
        self.min_increment = (2 * self.min_increment).min(MAX_MMAP);

        if new_mem == self.brk_end {
            // We allocated contiguous memory, just extend the current region
            self.brk_end += incr;
        } else {
            // We allocated non-contiguous memory, start a new region
            self.brk = new_mem;
            self.brk_start = new_mem;
            self.brk_end = new_mem + incr;
        }

        Ok(())
    }

    fn extend(&mut self, incr: usize) -> Result<()> {
        let incr = align_up(incr, GRANULARITY);
        loop {
            match self.extend_impl(incr) {
                Err(_) if self.min_increment > incr => {
                    self.min_increment /= 2;
                }
                res => return res,
            }
        }
    }
}

#[unsafe(no_mangle)]
extern "C" fn sbrk(incr: isize) -> *mut c_void {
    let mut brk = BRK.lock();

    if incr == 0 {
        return brk.brk as *mut c_void;
    }

    if incr < 0 {
        let neg_incr = (-incr) as usize;
        if brk.brk - neg_incr < brk.brk_start {
            // Requested a shrink beyond the current mmaped region
            Errno::ENOMEM.set_errno();
            return (-1isize) as *mut c_void;
        }
        let old_brk = brk.brk;
        brk.brk -= neg_incr;
        return old_brk as *mut c_void;
    }

    // incr > 0

    let incr = incr as usize;

    // If the sbrk comes from malloc, it will check that the storage is aligned,
    // and if it is not it will call sbrk again to extend the storage to account for the alignment.
    // If the second call fails it will result in ENOMEM, so we account for the second call already
    // here to make sure that it will not fail.
    let align_p = align_up(brk.brk + MALLOC_HEAD, MALLOC_CHUNK_ALIGN) - MALLOC_HEAD;
    let mut align_incr = incr;
    if align_p != brk.brk {
        align_incr += align_p - brk.brk;
    }

    if let Err(err) = brk.ensure_capacity(align_incr) {
        // We failed to ensure capacity, which means that we are out of memory
        err.set_errno();
        return (-1isize) as *mut c_void;
    }

    let old_brk = brk.brk;
    brk.brk += incr;

    old_brk as *mut c_void
}
