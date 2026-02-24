use core::ptr::NonNull;

use crate::emutls::EmutlsControl;

#[unsafe(no_mangle)]
extern "C" fn __emutls_get_address(control: &EmutlsControl<u8>) -> NonNull<u8> {
    control.get().into()
}
