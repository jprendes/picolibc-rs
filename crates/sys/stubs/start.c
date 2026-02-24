#include <unistd.h>

_Noreturn void rust_start();

_Noreturn void _start(void);

__attribute__((force_align_arg_pointer))
_Noreturn void _start(void) {
    rust_start();
    _exit(0);
}
