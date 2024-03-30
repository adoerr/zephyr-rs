#![no_std]

use panic_halt as _;
use zephyr_sys::printk;

#[no_mangle]
pub extern "C" fn rust_main() {
    unsafe {
        printk("\tHello World from Rust\n\0".as_ptr());
    }
}
