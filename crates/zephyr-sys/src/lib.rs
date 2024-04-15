#![no_std]

extern crate alloc;

mod log;

extern "C" {
    pub fn k_str_out(msg: *const u8, len: usize);
}

pub use log::init;
