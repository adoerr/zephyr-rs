#![no_std]

use panic_halt as _;

extern "C" {
    pub fn printk(text: *const u8);
}

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {
        printk("\tHello World from Rust\n\0".as_ptr());
    }
}
