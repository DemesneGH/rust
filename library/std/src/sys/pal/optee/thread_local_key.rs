// use crate::boxed::Box;
// use crate::ptr;

// pub type Key = usize;

// struct Allocated {
//     value: *mut u8,
//     dtor: Option<unsafe extern fn(*mut u8)>,
// }

// #[inline]
// pub unsafe fn create(dtor: Option<unsafe extern fn(*mut u8)>) -> Key {
//     Box::into_raw(Box::new(Allocated {
//         value: ptr::null_mut(),
//         dtor,
//     })) as usize
// }

// #[inline]
// pub unsafe fn set(key: Key, value: *mut u8) {
//     let ptr = crate::ptr::from_exposed_addr_mut(key as usize) as *mut Allocated;
//     (*ptr).value = value;
// }

// #[inline]
// pub unsafe fn get(key: Key) -> *mut u8 {
//     let ptr = crate::ptr::from_exposed_addr_mut(key as usize) as *mut Allocated;
//     (*ptr).value
// }

// #[inline]
// pub unsafe fn destroy(key: Key) {
//     let ptr = crate::ptr::from_exposed_addr_mut(key as usize);
//     let key = Box::from_raw(ptr);
//     if let Some(f) = key.dtor {
//         f(key.value);
//     }
// }

// #[inline]
// pub fn requires_synchronized_create() -> bool {
//     false
// }


use crate::alloc::{alloc, Layout};

pub type Key = usize;

#[inline]
pub unsafe fn create(_dtor: Option<unsafe extern "C" fn(*mut u8)>) -> Key {
    alloc(Layout::new::<*mut u8>()) as _
}

#[inline]
pub unsafe fn set(key: Key, value: *mut u8) {
    let key: *mut *mut u8 = core::ptr::with_exposed_provenance_mut(key);
    *key = value;
}

#[inline]
pub unsafe fn get(key: Key) -> *mut u8 {
    let key: *mut *mut u8 = core::ptr::with_exposed_provenance_mut(key);
    *key
}

#[inline]
pub unsafe fn destroy(_key: Key) {}
