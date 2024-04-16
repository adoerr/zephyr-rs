//!
//! zephyr-sys - Raw FFI bindings to Zephyr RTOS
//!

#![no_std]
#![deny(clippy::all)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

/// Tick precision used in timeout APIs.
pub type k_ticks_t = i64;

/// Kernel timeout type.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct k_timeout_t {
    pub ticks: k_ticks_t,
}

/// Infinite timeout delay.
pub const K_FOREVER: k_timeout_t = k_timeout_t {
    ticks: -1 as k_ticks_t,
};

/// Null timeout delay
pub const K_NO_WAIT: k_timeout_t = k_timeout_t { ticks: 0 };

extern "C" {
    pub fn k_str_out(msg: *const u8, len: usize);
}
