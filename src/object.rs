use crate::internal;

pub struct Object { id: internal::ObjectId }

impl Object {
    pub fn from_name(name: &str) -> Self {
        let name = std::ffi::CString::new(name).unwrap();
        let id = unsafe { internal::get_object(name.as_ptr()) };
        Self { id }
    }

    pub fn id(&self) -> internal::ObjectId { self.id }

    #[allow(clippy::too_many_arguments)]
    pub fn set_transform(
        &mut self,
        x: f32, y: f32, z: f32,
        rx: f32, ry: f32, rz: f32,
        sx: f32, sy: f32, sz: f32,
    ) {
        unsafe { internal::set_transform(self.id, x, y, z, rx, ry, rz, sx, sy, sz) }
    }

    pub fn set_location(&mut self, x: f32, y: f32, z: f32) {
        unsafe { internal::set_location(self.id, x, y, z) }
    }

    pub fn set_scale(&mut self, x: f32, y: f32, z: f32) {
        unsafe { internal::set_scale(self.id, x, y, z) }
    }

    pub fn set_rotation(&mut self, x: f32, y: f32, z: f32) {
        unsafe { internal::set_rotation(self.id, x, y, z) }
    }
}
