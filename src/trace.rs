use crate::internal;

pub trait Traceable { fn trace(&self); }

impl Traceable for &str {
    fn trace(&self) {
        let string = std::ffi::CString::new(*self).unwrap();
        unsafe { internal::trace(string.as_ptr()) }
    }
}

impl Traceable for f32 { fn trace(&self) { unsafe { internal::tracef(*self) } } }

impl Traceable for i32 { fn trace(&self) { unsafe { internal::tracei(*self) } } }

#[macro_export]
macro_rules! trace {
    ($e:expr) => { crate::trace::Traceable::trace(&$e) };
}
