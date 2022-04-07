use std::ptr;

use crate::{bindgen, utils::Entity};

use super::{Engine, IndirectLight, Skybox};

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

    #[inline]
    pub unsafe fn add_entities(&mut self, entities: &[Entity]) -> &mut Self {
        for entity in entities {
            self.add_entity(entity);
        }
        self
    }

    #[inline]
    pub unsafe fn remove_entity(&mut self, entity: &Entity) -> &mut Self {
        bindgen::filament_Scene_remove(self.native_mut(), entity.native_owned());
        self
    }

    #[inline]
    pub unsafe fn remove_entities(&mut self, entities: &[Entity]) -> &mut Self {
        for entity in entities {
            self.remove_entity(entity);
        }
        self
    }

    #[inline]
    pub unsafe fn set_skybox(&mut self, skybox: &mut Skybox) -> &mut Self {
        bindgen::filament_Scene_setSkybox(self.native_mut(), skybox.native_mut());
        self
    }

    #[inline]
    pub unsafe fn set_indirect_light(&mut self, ibl: &mut IndirectLight) -> &mut Self {
        bindgen::filament_Scene_setIndirectLight(self.native_mut(), ibl.native_mut());
        self
    }
}
