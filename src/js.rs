use std::os::raw::c_char;

use crate::{internal, Object};

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn js_eval(string: &str) {
    let string = std::ffi::CString::new(string).unwrap();
    unsafe { internal::js_eval(string.as_ptr()) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn js_call_object(object: Object, f: *const c_char) {
    unsafe { internal::js_call_object(object.id(), f) }
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub fn js_call_static(path: &str, f: *const c_char) {
    let path = std::ffi::CString::new(path).unwrap();
    unsafe { internal::js_call_static(path.as_ptr(), f) }
}
