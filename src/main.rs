#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // this function is an entry point provided for the linker
    vga_buffer::test_print();

    loop {}
}

// Function called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


