#include <unistd.h>
#include <errno.h>

void *_brk(void * addr);

static char *__brk = NULL;

void *sbrk(ptrdiff_t incr)
{
    if (__brk == NULL) {
        __brk = _brk(NULL);
        if (__brk == (void *) -1) {
            errno = ENOMEM;
            return (void *) -1;
        }
    }

    if (incr == 0) {
        return __brk;
    }

    char *old_brk = __brk;
    char *new_brk = __brk + incr;

    if (_brk(new_brk) != new_brk) {
        errno = ENOMEM;
        return (void *) -1;
    }

    __brk = new_brk;
    
	return old_brk;
}
