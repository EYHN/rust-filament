use std::rc::Rc;

use filament_bindings::{filament_Engine_destroy20, utils_Entity, utils_EntityManager_create};

use crate::{filament::Engine, prelude::NativeHandle};

#[derive(Clone)]
pub struct Entity(pub(crate) Rc<utils_Entity>, Engine);

impl NativeHandle<utils_Entity> for Entity {
    fn native(&self) -> *const utils_Entity {
        self.0.as_ref()
    }

    fn native_mut(&mut self) -> *mut utils_Entity {
        self.0.as_ref() as *const _ as *mut _
    }
}

impl Entity {
    pub(crate) fn dangling(engine: Engine) -> Entity {
        Entity(Rc::new(Default::default()), engine)
    }

    pub(crate) fn native_owned(&self) -> utils_Entity {
        self.0.as_ref().clone()
    }
}

impl Drop for Entity {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let Some(e) = Rc::<utils_Entity>::get_mut(&mut self.0) {
                filament_Engine_destroy20(self.1.native_mut(), e.clone());
                utils_EntityManager_create(self.1.get_entity_manager().native_mut(), 1, e);
            }
        }
    }
}
