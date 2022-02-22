use std::{collections::HashMap, ptr};

use filament_bindings::{
    filament_Engine_destroy20, utils_EntityManager, utils_EntityManager_create,
    utils_EntityManager_destroy,
};

use crate::{
    filament::{Engine, WeakEngine},
    prelude::NativeHandle,
};

use super::{Entity, EntityIdentify};

pub struct EntityManager {
    native: ptr::NonNull<utils_EntityManager>,
    pub(crate) engine: WeakEngine,
    managed_entity: HashMap<EntityIdentify, Entity>,
}

impl NativeHandle<utils_EntityManager> for EntityManager {
    #[inline]
    fn native(&self) -> *const utils_EntityManager {
        self.native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut utils_EntityManager {
        self.native.as_ptr()
    }
}

impl EntityManager {
    #[inline]
    pub(crate) fn try_from_native(
        native: *mut utils_EntityManager,
        managed_entity: HashMap<EntityIdentify, Entity>,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(EntityManager {
            native: ptr,
            engine: WeakEngine::new(),
            managed_entity,
        })
    }

    pub fn create(&mut self) -> &mut Entity {
        let mut entity = Entity::dangling(self.engine.clone());
        unsafe { utils_EntityManager_create(self.native_mut(), 1, entity.native_ptr_mut()) };
        let identity = entity.identify;
        self.managed_entity.insert(identity, entity);
        let entity = self.managed_entity.get_mut(&identity).unwrap();
        entity
    }

    #[inline]
    pub(crate) fn engine(&self) -> Engine {
        self.engine.upgrade_engine().ok().unwrap()
    }

    pub fn remove(&mut self, entity: &mut Entity) {
        unsafe {
            filament_Engine_destroy20(self.engine().native_mut(), entity.native_owned());
            utils_EntityManager_destroy(self.native_mut(), 1, entity.native_ptr_mut());
        }
    }
}
