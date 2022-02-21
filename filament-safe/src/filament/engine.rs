use std::{mem, os::raw, ptr, rc::Rc};

use filament_bindings::{
    filament_Engine, filament_Engine_flush, filament_Engine_flushAndWait,
    filament_Engine_getBackend, filament_Engine_getEntityManager, filament_Engine_getPlatform,
    filament_Engine_getRenderableManager, filament_Engine_pumpMessageQueues, filament_RenderableManager,
};

use crate::{
    backend::{Backend, Platform},
    prelude::NativeHandle,
    utils,
};

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
        Backend::from(unsafe { filament_Engine_getBackend(self.native()) })
    }

    #[inline]
    pub fn get_platform(&self) -> &mut Platform {
        unsafe { mem::transmute(filament_Engine_getPlatform(self.native())) }
    }

    #[inline]
    pub(crate) fn get_renderable_manger(&mut self) -> *mut filament_RenderableManager {
        unsafe { filament_Engine_getRenderableManager(self.native_mut()) }
    }
}

// TODO: drop