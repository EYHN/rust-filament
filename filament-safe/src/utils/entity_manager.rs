use std::{ptr, rc::Rc};

use filament_bindings::{utils_EntityManager, utils_EntityManager_create};

use crate::{filament::Engine, prelude::NativeHandle};

use super::Entity;

pub struct EntityManager(ptr::NonNull<utils_EntityManager>, Engine);

impl NativeHandle<utils_EntityManager> for EntityManager {
    #[inline]
    fn native(&self) -> *const utils_EntityManager {
        self.0.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut utils_EntityManager {
        self.0.as_ptr()
    }
}

impl EntityManager {
    #[inline]
    pub(crate) fn try_from_native(
        engine: Engine,
        native: *mut utils_EntityManager,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(EntityManager(ptr, engine))
    }

    pub fn create(&mut self) -> Entity {
        let mut entity = Entity::dangling(self.1.clone());
        unsafe {
            utils_EntityManager_create(self.native_mut(), 1, Rc::get_mut(&mut entity.0).unwrap())
        };
        entity
    }
}
