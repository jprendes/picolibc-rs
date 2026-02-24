#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

// The definition of `__emutls_control` and the signature of `__emutls_get_address` are
// based on the implementation of emutls in LLVM:
// https://github.com/llvm/llvm-project/blob/4814b9c93a13d0480f2839c2b8a049c4708e46d8/compiler-rt/lib/builtins/emutls.c

/* For every TLS variable xyz,
 * there is one __emutls_control variable named __emutls_v.xyz.
 * If xyz has non-zero initial value, __emutls_v.xyz's "value"
 * will point to __emutls_t.xyz, which has the initial value.
 */
typedef struct __emutls_control {
    size_t size;  /* size of the object in bytes */
    size_t align;  /* alignment of the object in bytes */
    union {
        uintptr_t index;  /* data[index-1] is the object address */
        void* address;  /* object address, when in single thread env */
    } object;
    void* value;  /* null or non-zero initial value for the object */
} __emutls_control;

// The entry point we will use from rust
extern void *__tls_get_address(__emutls_control* control);

void *__emutls_get_address(__emutls_control* control);

void *__emutls_get_address(__emutls_control* control) {
    return __tls_get_address(control);
}
