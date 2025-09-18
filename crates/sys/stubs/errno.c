#include <errno.h>

int* __picolibc_errno_location(void);

int* __picolibc_errno_location(void) {
    return &errno;
}