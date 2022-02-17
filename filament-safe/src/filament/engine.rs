use std::{os::raw, ptr};

use filament_bindings::{
    filament_BufferObject, filament_ColorGrading, filament_Engine, filament_Engine_createSwapChain,
    filament_Engine_createSwapChain1, filament_Engine_destroy10, filament_Engine_destroy11,
    filament_Engine_destroy12, filament_Engine_destroy13, filament_Engine_destroy14,
    filament_Engine_destroy15, filament_Engine_destroy16, filament_Engine_destroy17,
    filament_Engine_destroy18, filament_Engine_destroy19, filament_Engine_destroy2,
    filament_Engine_destroy3, filament_Engine_destroy4, filament_Engine_destroy5,
    filament_Engine_destroy6, filament_Engine_destroy7, filament_Engine_destroy8,
    filament_Engine_destroy9, filament_Engine_enableAccurateTranslations, filament_Engine_flush,
    filament_Engine_flushAndWait, filament_Engine_getBackend, filament_Engine_getPlatform,
    filament_Engine_pumpMessageQueues, filament_Fence, filament_IndexBuffer,
    filament_IndirectLight, filament_Material, filament_MaterialInstance,
    filament_MorphTargetBuffer, filament_RenderTarget, filament_Renderer, filament_Scene,
    filament_SkinningBuffer, filament_Skybox, filament_Stream, filament_SwapChain,
    filament_Texture, filament_VertexBuffer, filament_View,
};

use crate::{
    backend::{Backend, Platform},
    utils,
};

use super::{
    Camera, EngineDestroy, Fence, LightManager, RenderableManager, Renderer, Scene, SwapChain,
    SwapChainConfig, View,
};

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
    pub fn get_backend(&self) -> Backend {
        Backend::try_from(unsafe { filament_Engine_getBackend(self.0) }).unwrap()
    }

    #[inline]
    pub fn get_platform(&self) -> &Platform {
        unsafe { std::mem::transmute(filament_Engine_getPlatform(self.0)) }
    }

    #[inline]
    fn destroy(engine: &mut Engine) {
        unsafe {
            filament_Engine::destroy(engine.0 as *mut *mut filament_Engine);
        }
    }
}

macro_rules! engine_destory_type {
    ($native_type:ident, $destory_fn:ident) => {
        impl EngineDestroy for $native_type {
            fn destory(p: *const Self, engine: &mut super::Engine) -> bool {
                unsafe { $destory_fn(engine.0, p) }
            }
        }
    };
}

engine_destory_type!(filament_BufferObject, filament_Engine_destroy2);
engine_destory_type!(filament_VertexBuffer, filament_Engine_destroy3);
engine_destory_type!(filament_Fence, filament_Engine_destroy4);
engine_destory_type!(filament_IndexBuffer, filament_Engine_destroy5);
engine_destory_type!(filament_SkinningBuffer, filament_Engine_destroy6);
engine_destory_type!(filament_MorphTargetBuffer, filament_Engine_destroy7);
engine_destory_type!(filament_IndirectLight, filament_Engine_destroy8);
engine_destory_type!(filament_Material, filament_Engine_destroy9);
engine_destory_type!(filament_MaterialInstance, filament_Engine_destroy10);
engine_destory_type!(filament_Renderer, filament_Engine_destroy11);
engine_destory_type!(filament_Scene, filament_Engine_destroy12);
engine_destory_type!(filament_Skybox, filament_Engine_destroy13);
engine_destory_type!(filament_ColorGrading, filament_Engine_destroy14);
engine_destory_type!(filament_SwapChain, filament_Engine_destroy15);
engine_destory_type!(filament_Stream, filament_Engine_destroy16);
engine_destory_type!(filament_Texture, filament_Engine_destroy17);
engine_destory_type!(filament_RenderTarget, filament_Engine_destroy18);
engine_destory_type!(filament_View, filament_Engine_destroy19);

impl Drop for Engine {
    fn drop(&mut self) {
        Engine::destroy(self);
    }
}
