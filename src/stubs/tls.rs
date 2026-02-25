use core::ptr::NonNull;

use crate::emutls::EmutlsControl;
use crate::host::HOST;

#[unsafe(no_mangle)]
extern "C" fn __emutls_get_address(control: &EmutlsControl<u8>) -> NonNull<u8> {
    let thread_id = HOST.thread_id().expect("TLS not supported");
    control.get(thread_id).into()
}
