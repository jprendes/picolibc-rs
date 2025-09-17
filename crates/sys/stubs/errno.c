#include <errno.h>

int *_get_errno_ptr(void);

int *_get_errno_ptr() {
    return &errno;
}
