#![no_std]

extern "C" {
    pub fn printk(text: *const u8);
}
