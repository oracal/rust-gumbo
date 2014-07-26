use libc::c_void;
use ffi = super::ffi;
use std::mem;

pub fn gumbo_vector_to_vector<T>(gumbo_vector: &ffi::GumboVector) -> Vec<&T> {
    let mut vec = Vec::new();
    if gumbo_vector.data.is_not_null() {
        for index in range(0i, gumbo_vector.length as int) {
            unsafe {
                vec.push(&*mem::transmute::<*mut c_void, *mut T>(*(gumbo_vector.data.offset(index))));
            }
        }
    }
    vec
}
