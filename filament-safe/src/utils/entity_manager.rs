use std::mem;

use filament_bindings::{utils_EntityManager, utils_EntityManager_create};

use super::Entity;

pub struct EntityManager;

impl EntityManager {
    pub fn as_ptr(&self) -> *const utils_EntityManager {
        unsafe { mem::transmute(self) }
    }

    pub fn as_ptr_mut(&mut self) -> *mut utils_EntityManager {
        unsafe { mem::transmute(self) }
    }

    pub fn create(&mut self) -> Entity {
        let mut entity = Entity(self, Default::default());
        unsafe { utils_EntityManager_create(self.as_ptr_mut(), 1, &mut entity.1 as *mut _) };
        entity
    }
}
