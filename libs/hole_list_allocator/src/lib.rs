#![feature(allocator)]
#![allocator]
#![no_std]

#![feature(const_fn)]

use spin::Mutex;
use linked_list_allocator::Heap;

extern crate spin;
extern crate linked_list_allocator;
#[macro_use]
extern crate lazy_static;

pub const HEAP_START: usize = 0x_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

lazy_static! {
    static ref HEAP: Mutex<Heap> = Mutex::new(
        unsafe {
            Heap::new(HEAP_START, HEAP_SIZE)
        });
}

#[no_mangle]
pub extern fn __rust_allocate(size: usize, align: usize) -> *mut u8 {
    HEAP.lock().allocate_first_fit(size, align).expect("out of memory")
}

#[no_mangle]
pub extern fn __rust_deallocate(ptr: *mut u8, old_size: usize, align: usize) {
    unsafe {
        HEAP.lock().deallocate(ptr, old_size, align);
    }
}

#[no_mangle]
pub extern fn __rust_reallocate(ptr: *mut u8, _old_size: usize, size: usize,
                                align: usize) -> *mut u8 {
    use core::{ptr, cmp};

    let new_ptr = __rust_allocate(size, align);
    unsafe {ptr::copy(ptr, new_ptr, cmp::min(size, size))}
    __rust_deallocate(ptr, size, align);
    new_ptr
}

#[no_mangle]
pub extern fn __rust_reallocate_inplace(_ptr: *mut u8, old_size: usize,
                                        _size: usize, _align: usize) -> usize {
    old_size
}

#[no_mangle]
pub extern fn __rust_usable_size(size: usize, _align: usize) -> usize {
    size
}
