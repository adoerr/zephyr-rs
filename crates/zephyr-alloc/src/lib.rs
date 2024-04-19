use core::alloc::{GlobalAlloc, Layout};

use zephyr_sys as sys;

pub struct Mempool(pub &'static sys::k_heap);

unsafe impl Send for Mempool {}
unsafe impl Sync for Mempool {}

unsafe impl GlobalAlloc for Mempool {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let kheap = self.0 as *const _ as *mut _;

        let ret = sys::k_heap_alloc(kheap, layout.size(), sys::K_NO_WAIT) as *mut _;

        if ret as usize & (layout.align() - 1) != 0 {
            sys::printk("Rust unsatisfied alloc alignment\n\0".as_ptr() as *const core::ffi::c_char);
            core::ptr::null_mut()
        } else {
            ret
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let kheap = self.0 as *const _ as *mut _;
        sys::k_heap_free(kheap, ptr as *mut _)
    }
}

/// Assign a Zephyr k_heap as #[global_allocator]
///
/// This should be defined with K_HEAP_DEFINE and granted permission to any
/// Rust threads that need to use alloc.
#[macro_export]
macro_rules! global_mem_pool {
    ($pool:ident) => {
        extern "C" {
            static $pool: ::zephyr_sys::k_heap;
        }

        #[global_allocator]
        static GLOBAL: $crate::Mempool = $crate::Mempool(unsafe { &$pool });
    };
}
