use std::{cell::UnsafeCell, rc::Rc};

use filament_bindings::{
    filament_Engine_createCamera, filament_Engine_destroy20,
    filament_Engine_destroyCameraComponent, utils_Entity, utils_EntityManager_create,
};

use crate::{
    filament::{Camera, Engine, Renderable, RenderableOptions},
    prelude::NativeHandle,
};

struct EntityComponents {
    camera: Option<Camera>,
    renderable: Option<Renderable>,
}

#[derive(Clone)]
pub struct Entity {
    pub(crate) native: Rc<utils_Entity>,
    pub(crate) engine: Engine,

    components: Rc<UnsafeCell<EntityComponents>>,
}

impl NativeHandle<utils_Entity> for Entity {
    fn native(&self) -> *const utils_Entity {
        self.native.as_ref()
    }

    fn native_mut(&mut self) -> *mut utils_Entity {
        self.native.as_ref() as *const _ as *mut _
    }
}

impl Entity {
    #[inline]
    pub(crate) fn dangling(engine: Engine) -> Entity {
        Entity {
            native: Rc::new(Default::default()),
            engine,

            components: Rc::new(UnsafeCell::new(EntityComponents {
                camera: None,
                renderable: None,
            })),
        }
    }

    #[inline]
    pub(crate) fn native_owned(&self) -> utils_Entity {
        self.native.as_ref().clone()
    }

    #[inline]
    pub fn create_camera_component(&mut self) -> Option<&mut Camera> {
        self.delete_camera_component();
        let camera = unsafe {
            Camera::try_from_native(filament_Engine_createCamera(
                self.engine.native_mut(),
                self.native_owned(),
            ))
        };
        let components = unsafe { &mut *self.components.get() };
        components.camera = camera;
        components.camera.as_mut()
    }

    #[inline]
    pub fn get_camera_component(&self) -> Option<&Camera> {
        let components = unsafe { &mut *self.components.get() };
        components.camera.as_ref()
    }

    #[inline]
    pub fn get_camera_component_mut(&mut self) -> Option<&mut Camera> {
        let components = unsafe { &mut *self.components.get() };
        components.camera.as_mut()
    }

    #[inline]
    pub fn delete_camera_component(&mut self) -> &mut Self {
        let components = unsafe { &mut *self.components.get() };
        if let Some(_) = components.camera {
            unsafe {
                filament_Engine_destroyCameraComponent(
                    self.engine.native_mut(),
                    self.native_owned(),
                )
            };
            components.camera = None;
        }
        self
    }

    #[inline]
    pub fn create_renderable_component(&mut self, renderable_options: &mut RenderableOptions) -> Option<&mut Renderable> {
        self.delete_renderable_component();
        let renderable = Renderable::from_options(self, renderable_options);
        let components = unsafe { &mut *self.components.get() };
        components.renderable = renderable;
        components.renderable.as_mut()
    }

    #[inline]
    pub fn get_renderable_component(&self) -> Option<&Renderable> {
        let components = unsafe { &mut *self.components.get() };
        components.renderable.as_ref()
    }

    #[inline]
    pub fn get_renderable_component_mut(&mut self) -> Option<&mut Renderable> {
        let components = unsafe { &mut *self.components.get() };
        components.renderable.as_mut()
    }
    
    #[inline]
    pub fn delete_renderable_component(&mut self) -> &mut Self {
        let components = unsafe { &mut *self.components.get() };
        if let Some(_) = components.renderable {
            Renderable::destroy_instance(self);
            components.renderable = None;
        }
        self
    }
}

impl Drop for Entity {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            if let Some(e) = Rc::<utils_Entity>::get_mut(&mut self.native) {
                filament_Engine_destroy20(self.engine.native_mut(), e.clone());
                utils_EntityManager_create(self.engine.get_entity_manager().native_mut(), 1, e);
            }
        }
    }
}
