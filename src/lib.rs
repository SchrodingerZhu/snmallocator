#![no_std]
extern crate snmalloc_sys as ffi;

use core::alloc::{GlobalAlloc, Layout};

pub struct SnMalloc;

unsafe impl GlobalAlloc for SnMalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ffi::for_rust_memalign(layout.align(), layout.size()) as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ffi::for_rust_free(ptr as _);
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ffi::for_rust_realloc_aligned(ptr as _, new_size, layout.align()) as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_frees_allocated_memory() {
        unsafe {
            let layout = Layout::from_size_align(8, 8).unwrap();
            let alloc = SnMalloc;

            let ptr = alloc.alloc(layout.clone());
            alloc.dealloc(ptr, layout);
        }
    }

    #[test]
    fn it_frees_zero_allocated_memory() {
        unsafe {
            let layout = Layout::from_size_align(8, 8).unwrap();
            let alloc = SnMalloc;

            let ptr = alloc.alloc_zeroed(layout.clone());
            alloc.dealloc(ptr, layout);
        }
    }

    #[test]
    fn it_frees_reallocated_memory() {
        unsafe {
            let layout = Layout::from_size_align(8, 8).unwrap();
            let alloc = SnMalloc;

            let ptr = alloc.alloc(layout.clone());
            let ptr = alloc.realloc(ptr, layout.clone(), 16);
            alloc.dealloc(ptr, layout);
        }
    }
}