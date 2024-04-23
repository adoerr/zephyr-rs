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

/// Kernel synchronized heap structure.
#[repr(C)]
pub struct k_heap {
    _unused: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    /// Allocate memory from a [k_heap].
    pub fn k_heap_alloc(heap: *mut k_heap, size: usize, timeout: k_timeout_t) -> *mut u8;
    /// Free memory allocated by [k_heap_alloc].
    pub fn k_heap_free(heap: *mut k_heap, mem: *mut u8);

    /// Print kernel debugging message.
    pub fn printk(fmt: *const core::ffi::c_char);
}
