use crate::internal;

pub fn time_real() -> f32 {
    unsafe { internal::time_real() }
}

pub fn time_delta() -> f32 {
    unsafe { internal::time_delta() }
}
