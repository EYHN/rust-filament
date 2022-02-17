use std::{os::raw, ptr};

use filament_bindings::{
    filament_Engine, filament_Engine_createSwapChain, filament_Engine_enableAccurateTranslations, filament_Engine_createSwapChain1, filament_Engine_flushAndWait, filament_Engine_flush, filament_Engine_pumpMessageQueues,
};

use crate::{backend::Backend, utils};

use super::{LightManager, RenderableManager, SwapChain, SwapChainConfig, Renderer, View, Scene, Camera, Fence};

pub struct Engine(pub(crate) *mut filament_Engine);

impl Engine {
    pub(crate) fn from_ptr(raw_ptr: *mut filament_Engine) -> Option<Self> {
        if raw_ptr.is_null() {
            None
        } else {
            Some(Self(raw_ptr))
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
    pub fn create_swap_chain(
        &mut self,
        native_window: *mut raw::c_void,
        flags: SwapChainConfig,
    ) -> Option<SwapChain> {
        unsafe {
            SwapChain::from_ptr(filament_Engine_createSwapChain(
                self.0,
                native_window,
                flags.bits(),
            ))
        }
    }

    #[inline]
    pub fn create_headless_swap_chain(
        &mut self,
        width: u32,
        height: u32,
        flags: SwapChainConfig,
    ) -> Option<SwapChain> {
        unsafe {
            SwapChain::from_ptr(filament_Engine_createSwapChain1(
                self.0,
                width,
                height,
                flags.bits(),
            ))
        }
    }

    #[inline]
    pub fn create_renderer(&mut self) -> Option<Renderer> {
        todo!()
    }

    #[inline]
    pub fn create_view(&mut self) -> Option<View> {
        todo!()
    }

    #[inline]
    pub fn create_scene(&mut self) -> Option<Scene> {
        todo!()
    }

    #[inline]
    pub fn create_camera(&mut self) -> Option<&Camera> {
        todo!()
    }

    #[inline]
    pub fn create_camera_component(&mut self) -> Option<&Camera> {
        todo!()
    }

    #[inline]
    pub fn create_fence(&mut self) -> Option<Fence> {
        todo!()
    }

    #[inline]
    pub fn flush_and_wait(&mut self) {
        unsafe { filament_Engine_flushAndWait(self.0) };
    }

    #[inline]
    pub fn flush(&mut self) {
        unsafe { filament_Engine_flush(self.0) };
    }

    #[inline]
    pub fn pump_message_queues(&mut self) {
        unsafe { filament_Engine_pumpMessageQueues(self.0) };
    }

    #[inline]
    pub fn get_default_material(&mut self) {
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
