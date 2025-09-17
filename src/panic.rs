use crate::eprintln;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    eprintln!("\n{info}");
    unsafe {
        picolibc_sys::exit(137);
    }
}
