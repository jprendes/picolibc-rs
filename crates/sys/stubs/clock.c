#define _POSIX_MONOTONIC_CLOCK 1
#define _POSIX_C_SOURCE 200809L

#include <sys/time.h>
#include <time.h>
#include <errno.h>
#include <stddef.h>
#include <stdint.h>

int gettimeofday(struct timeval *__restrict tv, void *__restrict __tz) {
    (void)__tz;

    struct timespec ts;
    clock_gettime(CLOCK_REALTIME, &ts);
    tv->tv_sec = ts.tv_sec;
    tv->tv_usec = ts.tv_nsec / 1000;

    return 0;
}

