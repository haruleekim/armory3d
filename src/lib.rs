use std::os::raw::{c_char, c_int};

use internal::{ButtonId, ObjectId, UpdateFunc};

mod internal {
    use std::os::raw::{c_char, c_int};

    pub type ObjectId = c_int;
    pub type ButtonId = c_int;
    pub type UpdateFunc = extern "C" fn();

    extern {
        pub fn get_object(name: *const c_char) -> ObjectId;
        pub fn notify_on_update(f: extern fn());
        pub fn remove_update(f: UpdateFunc);
        pub fn set_transform(
            object: ObjectId,
            x: f32, y: f32, z: f32,
            rx: f32, ry: f32, rz: f32,
            sx: f32, sy: f32, sz: f32,
        );
        pub fn set_location(object: ObjectId, x: f32, y: f32, z: f32);
        pub fn set_scale(object: ObjectId, x: f32, y: f32, z: f32);
        pub fn set_rotation(object: ObjectId, x: f32, y: f32, z: f32);
        pub fn mouse_x() -> c_int;
        pub fn mouse_y() -> c_int;
        pub fn mouse_started(button: ButtonId) -> c_int;
        pub fn mouse_down(button: ButtonId) -> c_int;
        pub fn mouse_released(button: ButtonId) -> c_int;
        pub fn key_started(key: c_int) -> c_int;
        pub fn key_down(key: c_int) -> c_int;
        pub fn key_released(key: c_int) -> c_int;
        pub fn time_real() -> f32;
        pub fn time_delta() -> f32;
        pub fn js_eval(f: *const c_char);
        pub fn js_call_object(object: ObjectId, f: *const c_char);
        pub fn js_call_static(path: *const c_char, f: *const c_char);
    }

    extern "C" {
        pub fn trace(s: *const c_char);
        pub fn tracef(f: f32);
        pub fn tracei(i: c_int);
    }
}

pub fn get_object_by_name(name: &str) -> ObjectId {
    let name = std::ffi::CString::new(name).unwrap();
    unsafe { internal::get_object(name.as_ptr()) }
}

pub fn notify_on_update(f: UpdateFunc) {
    unsafe { internal::notify_on_update(f) }
}

pub fn remove_update(f: UpdateFunc) {
    unsafe { internal::remove_update(f) }
}

pub fn set_transform(
    object: i32,
    x: f32, y: f32, z: f32,
    rx: f32, ry: f32, rz: f32,
    sx: f32, sy: f32, sz: f32,
) {
    unsafe { internal::set_transform(object, x, y, z, rx, ry, rz, sx, sy, sz) }
}

pub fn set_location(object: ObjectId, x: f32, y: f32, z: f32) {
    unsafe { internal::set_location(object, x, y, z) }
}

pub fn set_scale(object: ObjectId, x: f32, y: f32, z: f32) {
    unsafe { internal::set_scale(object, x, y, z) }
}

pub fn set_rotation(object: ObjectId, x: f32, y: f32, z: f32) {
    unsafe { internal::set_rotation(object, x, y, z) }
}

pub fn mouse_x() -> c_int {
    unsafe { internal::mouse_x() }
}

pub fn mouse_y() -> c_int {
    unsafe { internal::mouse_y() }
}

pub fn mouse_started(button: ButtonId) -> c_int {
    unsafe { internal::mouse_started(button) }
}

pub fn mouse_down(button: ButtonId) -> c_int {
    unsafe { internal::mouse_down(button) }
}

pub fn mouse_released(button: ButtonId) -> c_int {
    unsafe { internal::mouse_released(button) }
}

pub fn key_started(key: c_int) -> c_int {
    unsafe { internal::key_started(key) }
}

pub fn key_down(key: c_int) -> c_int {
    unsafe { internal::key_down(key) }
}

pub fn key_released(key: c_int) -> c_int {
    unsafe { internal::key_released(key) }
}

pub fn time_real() -> f32 {
    unsafe { internal::time_real() }
}

pub fn time_delta() -> f32 {
    unsafe { internal::time_delta() }
}

pub fn js_eval(f: *const c_char) {
    unsafe { internal::js_eval(f) }
}

pub fn js_call_object(object: ObjectId, f: *const c_char) {
    unsafe { internal::js_call_object(object, f) }
}

pub fn js_call_static(path: &str, f: *const c_char) {
    let path = std::ffi::CString::new(path).unwrap();
    unsafe { internal::js_call_static(path.as_ptr(), f) }
}

pub fn trace_s(string: &str) {
    let string = std::ffi::CString::new(string).unwrap();
    unsafe { internal::trace(string.as_ptr()) }
}

pub fn trace_f(float: f32) {
    unsafe { internal::tracef(float) }
}

pub fn trace_i(int: c_int) {
    unsafe { internal::tracei(int) }
}

pub trait Traceable { fn trace(&self); }

impl Traceable for &str { fn trace(&self) { trace_s(*self) } }

impl Traceable for f32 { fn trace(&self) { trace_f(*self) } }

impl Traceable for i32 { fn trace(&self) { trace_i(*self) } }

#[macro_export]
macro_rules! trace {
    ($e:expr) => { Traceable::trace(&$e) };
}
