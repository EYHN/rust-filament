use std::{mem, os::raw, ptr};

use filament_bindings::{
    filament_Engine, filament_Engine_flush, filament_Engine_flushAndWait,
    filament_Engine_getBackend, filament_Engine_getEntityManager, filament_Engine_getPlatform,
    filament_Engine_getRenderableManager, filament_Engine_pumpMessageQueues,
    filament_RenderableManager,
};

use crate::{
    backend::{Backend, Platform},
    prelude::{EngineError, EngineResult, RcHandle, WeakHandle},
    utils::{self, EntityManager},
};

pub struct EngineInner {
    native: ptr::NonNull<filament_Engine>,
    entity_manager: EntityManager,
}

pub type Engine = RcHandle<EngineInner>;
pub type WeakEngine = WeakHandle<EngineInner>;

impl WeakEngine {
    pub fn upgrade_engine(&self) -> EngineResult<Engine> {
        self.upgrade().ok_or(EngineError::EngineDestroyed)
    }
}

impl Engine {
    #[inline]
    pub fn native(&self) -> *const filament_Engine {
        self.as_ref().native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut filament_Engine {
        self.as_mut().native.as_ptr()
    }

    pub(crate) fn try_from_native(native: *mut filament_Engine) -> Option<Engine> {
        let ptr = ptr::NonNull::new(native)?;
        let entity_manager = EntityManager::try_from_native(
            unsafe { filament_Engine_getEntityManager(native) },
            Default::default(),
        )?;
        let mut engine = RcHandle::new(EngineInner {
            native: ptr,
            entity_manager,
        });
        engine.as_mut().entity_manager.engine = engine.downgrade();
        Some(engine)
    }

    #[inline]
    pub fn create(backend: Backend) -> Option<Engine> {
        Self::try_from_native(unsafe {
            filament_Engine::create(backend.into(), ptr::null_mut(), ptr::null_mut())
        })
    }

    #[inline]
    pub fn create_platform(backend: Backend, platform: &mut Platform) -> Option<Engine> {
        Self::try_from_native(unsafe {
            filament_Engine::create(backend.into(), mem::transmute(platform), ptr::null_mut())
        })
    }

    #[inline]
    pub fn create_shared_gl_context(
        backend: Backend,
        shared_gl_context: *mut raw::c_void,
    ) -> Option<Engine> {
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
    pub fn get_entity_manager(&mut self) -> &mut utils::EntityManager {
        &mut self.as_mut().entity_manager
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
    pub fn get_default_material(&self) {
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
