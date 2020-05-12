pub use internal::{ButtonId, ObjectId, UpdateFunc};
pub use object::Object;

pub mod input;
pub mod js;
pub mod time;
pub mod trace;

mod internal;
mod object;

pub fn notify_on_update(f: UpdateFunc) {
    unsafe { internal::notify_on_update(f) }
}

pub fn remove_update(f: UpdateFunc) {
    unsafe { internal::remove_update(f) }
}
