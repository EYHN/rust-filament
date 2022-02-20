use std::{ptr, rc::Rc};

use filament_bindings::{filament_Scene, filament_Engine_createScene, filament_Engine_destroy12};

use crate::prelude::NativeHandle;

use super::Engine;

#[derive(Clone)]
pub struct Scene {
    native: Rc<ptr::NonNull<filament_Scene>>,
    engine: Engine,
}

impl NativeHandle<filament_Scene> for Scene {
    #[inline]
    fn native(&self) -> *const filament_Scene {
        self.native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_Scene {
        self.native.as_ptr()
    }
}

impl Scene {
    #[inline]
    pub(crate) fn try_from_native(engine: Engine, native: *mut filament_Scene) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self {
            native: Rc::new(ptr),
            engine
        })
    }

    #[inline]
    pub fn create(engine: &mut Engine) -> Option<Self> {
        unsafe {
            Self::try_from_native(
                engine.clone(),
                filament_Engine_createScene(engine.native_mut()),
            )
        }
    }
}

impl Drop for Scene {
    #[inline]
    fn drop(&mut self) {
        if let Some(_) = Rc::get_mut(&mut self.native) {
            unsafe { filament_Engine_destroy12(self.engine.native_mut(), self.native()) };
        }
    }
}
