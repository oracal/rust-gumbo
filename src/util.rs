use libc::{c_char, size_t, c_void};
use std::str::raw::{from_c_str, from_buf_len};
use ffi = super::ffi;

pub fn cstr_to_option_string(cstr: *const c_char) -> Option<String> {
    unsafe {
        if cstr.is_null() {
            None
        } else {
            Some(from_c_str(cstr))
        }
    }
}

pub fn buffer_to_option_string(buffer: *const c_char, length: size_t) -> Option<String> {
    unsafe {
        if buffer.is_null() {
            None
        } else {
            Some(from_buf_len(buffer as *const u8, length as uint))
        }
    }
}

pub fn gumbo_vector_to_vector<T>(gumbo_vector: ffi::GumboVector) -> Vec<*mut T> {
    let mut vec = Vec::new();
    if gumbo_vector.data.is_not_null() {
        for index in range(0i, gumbo_vector.length as int) {
            unsafe {
                vec.push(::std::mem::transmute::<*mut c_void, *mut T>((*(gumbo_vector.data))).offset(index));
            }
        }
    }
    vec
}
