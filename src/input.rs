use std::os::raw::c_int;

use internal::ButtonId;

use crate::internal;

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
