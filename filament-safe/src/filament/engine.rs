use std::{ptr, os::raw};

use filament_bindings::{filament_Engine, filament_Engine_enableAccurateTranslations};

use crate::{backend::Backend, utils};

use super::{LightManager, RenderableManager, SwapChain};

pub struct Engine(*mut filament_Engine);

impl Engine {
    pub fn from_ptr(raw_ptr: *mut filament_Engine) -> Option<Engine> {
        if raw_ptr.is_null() {
            None
        } else {
            Some(Engine(raw_ptr))
        }
    }

    #[inline]
    pub fn create(backend: Backend) -> Option<Engine> {
        Engine::from_ptr(unsafe {
            filament_Engine::create(backend.into(), ptr::null_mut(), ptr::null_mut())
        })
    }

    // TODO: create_platform
    // TODO: create_shared_gl_context
    // TODO: create_platform_shared_gl_context

    // TODO: create_async

    #[inline]
    pub fn get_entity_manager_mut(&mut self) -> &mut utils::EntityManager {
        todo!()
    }

    #[inline]
    pub fn get_renderable_manager_mut(&mut self) -> &mut RenderableManager {
        todo!()
    }

    #[inline]
    pub fn get_light_manager_mut(&mut self) -> &mut LightManager {
        todo!()
    }

    #[inline]
    pub fn enable_accurate_translations(&mut self) {
        unsafe { filament_Engine_enableAccurateTranslations(self.0) }
    }

    #[inline]
    pub fn create_swap_chain(native_window: *const raw::c_void, flag: u64) -> Option<SwapChain> {
        todo!()
    }

    #[inline]
    pub fn create_headless_swap_chain(width: u32, height: u32, flag: u64) -> Option<SwapChain> {
        todo!()
    }

    #[inline]
    fn destroy(engine: &mut Engine) {
        unsafe {
            filament_Engine::destroy(engine.0 as *mut *mut filament_Engine);
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        Engine::destroy(self);
    }
}
