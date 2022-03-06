use std::hash;

use crate::bindgen;

#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct Entity {
    native: bindgen::utils_Entity,
}

impl hash::Hash for Entity {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.native.mIdentity.hash(state);
    }
}

impl Entity {
    #[inline]
    pub fn native_ptr(&self) -> *const bindgen::utils_Entity {
        &self.native
    }

    #[inline]
    pub fn native_ptr_mut(&mut self) -> *mut bindgen::utils_Entity {
        &mut self.native
    }

    #[inline]
    pub(crate) fn dangling() -> Entity {
        Entity {
            native: Default::default(),
        }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        self.native.mIdentity == 0
    }

    #[inline]
    pub(crate) fn native_owned(&self) -> bindgen::utils_Entity {
        self.native
    }

    #[inline]
    pub(crate) fn try_from_native(native: bindgen::utils_Entity) -> Option<Entity> {
        if native.mIdentity == 0 {
            None
        } else {
            Some(Entity { native })
        }
    }
}
