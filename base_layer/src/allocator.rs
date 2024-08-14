use alloc::alloc::{Layout, GlobalAlloc};
use core::ptr::null_mut;


pub struct LibcAllocator;

unsafe impl GlobalAlloc for LibcAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = libc::malloc(layout.size()) as *mut u8;
        if ptr.is_null() {
            null_mut()
        } else {
            ptr
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut libc::c_void)
    }
}