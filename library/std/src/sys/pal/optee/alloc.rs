use crate::alloc::{GlobalAlloc, Layout, System};
//use crate::libc;

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        libc::malloc(layout.size()) as *mut u8
    }

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        libc::calloc(layout.size(), 1) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        libc::free(ptr as *mut libc::c_void)
    }

    #[inline]
    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        libc::realloc(ptr as *mut libc::c_void, new_size) as *mut u8
    }

}
