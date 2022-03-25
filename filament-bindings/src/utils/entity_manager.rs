use std::ptr;

use crate::bindgen;

use super::Entity;

pub struct EntityManager {
    native: ptr::NonNull<bindgen::utils_EntityManager>,
}

impl EntityManager {
    #[inline]
    pub fn native(&self) -> *const bindgen::utils_EntityManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::utils_EntityManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::utils_EntityManager) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(EntityManager { native: ptr })
    }

    pub unsafe fn create(&mut self) -> Entity {
        let mut entity = Entity::dangling();
        bindgen::utils_EntityManager_create(self.native_mut(), 1, entity.native_ptr_mut());
        entity
    }

    pub unsafe fn destory(&mut self, entity: &mut Entity) {
        bindgen::utils_EntityManager_destroy(self.native_mut(), 0, entity.native_ptr_mut());
    }
}
