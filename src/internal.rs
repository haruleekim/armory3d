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
