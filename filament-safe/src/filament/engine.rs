use std::{mem, os::raw, ptr, rc::Rc};

use filament_bindings::{
    filament_Engine, filament_Engine_flush, filament_Engine_flushAndWait,
    filament_Engine_getBackend, filament_Engine_getEntityManager, filament_Engine_getPlatform,
    filament_Engine_pumpMessageQueues,
};

use crate::{
    backend::{Backend, Platform},
    prelude::NativeHandle,
    utils,
};

// impl EngineHandler {
//     #[inline]
//     pub(crate) fn try_from_ptr(raw_ptr: *mut filament_Engine) -> Option<Self> {
//         if raw_ptr.is_null() {
//             None
//         } else {
//             Some(Self(raw_ptr))
//         }
//     }

//     #[inline]
//     fn destroy(engine: &mut EngineHandler) {
//         unsafe {
//             filament_Engine::destroy(engine.0 as *mut *mut filament_Engine);
//         }
//     }
// }

// macro_rules! engine_destory_type {
//     ($native_type:ident, $destory_fn:ident) => {
//         impl NativeManager<*mut $native_type> for EngineHandler {
//             fn destroy(&mut self, p: *mut $native_type) {
//                 unsafe { $destory_fn(self.0, p) };
//             }
//         }
//     };
// }

// engine_destory_type!(filament_BufferObject, filament_Engine_destroy2);
// engine_destory_type!(filament_VertexBuffer, filament_Engine_destroy3);
// engine_destory_type!(filament_Fence, filament_Engine_destroy4);
// engine_destory_type!(filament_IndexBuffer, filament_Engine_destroy5);
// engine_destory_type!(filament_SkinningBuffer, filament_Engine_destroy6);
// engine_destory_type!(filament_MorphTargetBuffer, filament_Engine_destroy7);
// engine_destory_type!(filament_IndirectLight, filament_Engine_destroy8);
// engine_destory_type!(filament_Material, filament_Engine_destroy9);
// engine_destory_type!(filament_MaterialInstance, filament_Engine_destroy10);
// engine_destory_type!(filament_Renderer, filament_Engine_destroy11);
// engine_destory_type!(filament_Scene, filament_Engine_destroy12);
// engine_destory_type!(filament_Skybox, filament_Engine_destroy13);
// engine_destory_type!(filament_ColorGrading, filament_Engine_destroy14);
// engine_destory_type!(filament_SwapChain, filament_Engine_destroy15);
// engine_destory_type!(filament_Stream, filament_Engine_destroy16);
// engine_destory_type!(filament_Texture, filament_Engine_destroy17);
// engine_destory_type!(filament_RenderTarget, filament_Engine_destroy18);
// engine_destory_type!(filament_View, filament_Engine_destroy19);

// impl NativeManager<utils_Entity> for EngineHandler {
//     fn destroy(&mut self, p: utils_Entity) {
//         unsafe { filament_Engine_destroy20(self.0, p) };
//     }
// }

// impl Drop for EngineHandler {
//     fn drop(&mut self) {
//         EngineHandler::destroy(self);
//     }
// }

#[derive(Clone)]
pub struct Engine(pub(crate) Rc<ptr::NonNull<filament_Engine>>);

impl NativeHandle<filament_Engine> for Engine {
    #[inline]
    fn native(&self) -> *const filament_Engine {
        self.0.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_Engine {
        unsafe { mem::transmute(self.0.as_ptr()) }
    }
}

impl Engine {
    pub(crate) fn try_from_native(native: *mut filament_Engine) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Engine(Rc::new(ptr)))
    }

    #[inline]
    pub fn create(backend: Backend) -> Option<Self> {
        Self::try_from_native(unsafe {
            filament_Engine::create(backend.into(), ptr::null_mut(), ptr::null_mut())
        })
    }

    #[inline]
    pub fn create_platform(backend: Backend, platform: &mut Platform) -> Option<Self> {
        Self::try_from_native(unsafe {
            filament_Engine::create(backend.into(), mem::transmute(platform), ptr::null_mut())
        })
    }

    #[inline]
    pub fn create_shared_gl_context(
        backend: Backend,
        shared_gl_context: *mut raw::c_void,
    ) -> Option<Self> {
        Self::try_from_native(unsafe {
            filament_Engine::create(backend.into(), ptr::null_mut(), shared_gl_context)
        })
    }

    #[inline]
    pub fn create_platform_shared_gl_context(
        backend: Backend,
        platform: &mut Platform,
        shared_gl_context: *mut raw::c_void,
    ) -> Option<Engine> {
        Self::try_from_native(unsafe {
            filament_Engine::create(backend.into(), mem::transmute(platform), shared_gl_context)
        })
    }

    // TODO: create_async

    #[inline]
    pub fn get_entity_manager(&mut self) -> utils::EntityManager {
        unsafe {
            utils::EntityManager::try_from_native(
                self.clone(),
                filament_Engine_getEntityManager(self.native_mut()),
            )
            .unwrap()
        }
    }

    #[inline]
    pub fn flush_and_wait(&mut self) {
        unsafe { filament_Engine_flushAndWait(self.native_mut()) };
    }

    #[inline]
    pub fn flush(&mut self) {
        unsafe { filament_Engine_flush(self.native_mut()) };
    }

    #[inline]
    pub fn pump_message_queues(&mut self) {
        unsafe { filament_Engine_pumpMessageQueues(self.native_mut()) };
    }

    #[inline]
    pub fn get_default_material(&mut self) {
        todo!()
    }

    #[inline]
    pub fn get_backend(&self) -> Backend {
        Backend::try_from(unsafe { filament_Engine_getBackend(self.native()) }).unwrap()
    }

    #[inline]
    pub fn get_platform(&self) -> &mut Platform {
        unsafe { mem::transmute(filament_Engine_getPlatform(self.native())) }
    }
}
