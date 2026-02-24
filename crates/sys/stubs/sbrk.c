#include "../picolibc/newlib/libc/stdlib/nano-malloc.h"
#include <unistd.h>
#include <errno.h>
#include <sys/cdefs.h>

#define KiB (1024LL)
#define MiB (KiB * KiB)
#define GiB (KiB * MiB)
#define TiB (KiB * GiB)

#define HEAP_START (1 * TiB)  // start heap at 1 TiB
#define GRANULARITY (64 * KiB) // align brk to 64 KiB
#define MAX_MMAP (16 * MiB) // maximum size of a single mmap request

static char *__brk = (char *)HEAP_START;
static char *__brk_start = (char *)HEAP_START;
static char *__brk_end = (char *)HEAP_START;

extern void *_mmap(void *addr, size_t length);

static size_t min_increment = GRANULARITY;

static bool sbrk_extend_impl(size_t incr) {
    incr = MAX(incr, min_increment);
    char *new_mem = _mmap(__brk_end, incr);

    if (new_mem == NULL) {
        // we failed to allocate new memory, which means that we are out of memory
        return false;
    }

    // we manage to allocate new memory, increment the min_increment for the next time
    min_increment = MIN(2 * min_increment, MAX_MMAP);

    if (new_mem == __brk_end) {
        // we allocated contiguous memory,
        // just extend the current region
        __brk_end = __brk_end + incr;
    } else if (new_mem != NULL) {
        // we allocated non-contiguous memory,
        // start a new region
        __brk = __brk_start = new_mem;
        __brk_end = new_mem + incr;
    }
    return true;
}

static bool sbrk_extend(size_t incr) {
    incr = __align_up(incr, GRANULARITY);
    while (1) {
        if (sbrk_extend_impl(incr)) {
            return true;
        }
        if (min_increment > incr) {
            min_increment /= 2;
            continue;
        }
    }
    return false;
}

static bool sbrk_has_capacity(size_t incr) {
    return __brk + incr <= __brk_end;
}

static bool sbrk_ensure_capacity(size_t incr) {
    return sbrk_has_capacity(incr) || sbrk_extend(incr);
}

void *sbrk(ptrdiff_t incr) {
    if (incr == 0) {
        return __brk;
    }

    if (incr < 0) {
        if (__brk + incr < __brk_start) {
            // requested a shrink beyond the current mmaped region
            errno = ENOMEM;
            return (void *) -1;
        }
        char *old_brk = __brk;
        __brk += incr;
        return old_brk;
    }

    // incr > 0

    // If the sbrk comes from malloc, it will check that the storage is aligned,
    // and if it is not it will call sbrk again to extend the storage to account for the alignment.
    // If the second call fails it will result in ENOMEM, so we account for the second call already
    // here to make sure that it will not fail.
    char *align_p = __align_up(__brk + MALLOC_HEAD, MALLOC_CHUNK_ALIGN) - MALLOC_HEAD;
    ptrdiff_t align_incr = incr;
    if (align_p != __brk) {
        align_incr += align_p - __brk;
    }

    if (!sbrk_ensure_capacity(align_incr)) {
        // we failed to ensure capacity, which means that we are out of memory
        errno = ENOMEM;
        return (void *) -1;
    }

    char *old_brk = __brk;
    __brk += incr;
    
	return old_brk;
}
