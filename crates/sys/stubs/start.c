#include <unistd.h>

_Noreturn void rust_start();

_Noreturn void _start(void);

char HEAP[1024 * 1024 * 64]; // 32 MiB heap for sbrk

__attribute__((force_align_arg_pointer))
_Noreturn void _start(void) {
    rust_start();
    _exit(0);
}
