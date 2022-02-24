use std::ptr;

use crate::{bindgen, utils::Entity};

use super::Engine;

pub struct Scene {
    native: ptr::NonNull<bindgen::filament_Scene>,
}

impl Scene {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_Scene {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_Scene {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_Scene) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }

    #[inline]
    pub unsafe fn create(engine: &mut Engine) -> Option<Self> {
        Self::try_from_native(bindgen::filament_Engine_createScene(engine.native_mut()))
    }

    #[inline]
    pub unsafe fn add_entity(&mut self, entity: &Entity) -> &mut Self {
        bindgen::filament_Scene_addEntity(self.native_mut(), entity.native_owned());
        self
    }
}
