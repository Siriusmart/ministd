use crate::alloc::c_malloc;

pub fn malloc<T>(n: usize) -> *mut T {
    unsafe { c_malloc::malloc(n * size_of::<T>()) as *mut T }
}

pub fn free<T>(ptr: *mut T) {
    unsafe { c_malloc::free(ptr as *mut u8) }
}
