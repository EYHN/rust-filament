use std::hash;

use filament_bindings::{
    filament_Engine_createCamera, filament_Engine_destroyCameraComponent, utils_Entity,
};

use crate::filament::{Camera, Engine, Renderable, RenderableOptions, WeakEngine};

#[derive(Debug, Default, Clone, Copy)]
pub struct EntityIdentify {
    native: utils_Entity,
}

impl hash::Hash for EntityIdentify {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.native.mIdentity.hash(state);
    }
}

impl PartialEq for EntityIdentify {
    fn eq(&self, other: &Self) -> bool {
        self.native.mIdentity == other.native.mIdentity
    }
}
impl Eq for EntityIdentify {}

struct EntityComponents {
    camera: Option<Camera>,
    renderable: Option<Renderable>,
}

pub struct Entity {
    pub identify: EntityIdentify,
    engine: WeakEngine,
    components: EntityComponents,
}

impl Entity {
    #[inline]
    pub fn native_ptr(&self) -> *const utils_Entity {
        &self.identify.native
    }

    #[inline]
    pub fn native_ptr_mut(&mut self) -> *mut utils_Entity {
        &mut self.identify.native
    }

    #[inline]
    pub(crate) fn dangling(engine: WeakEngine) -> Entity {
        Entity {
            identify: Default::default(),
            engine,

            components: EntityComponents {
                camera: None,
                renderable: None,
            },
        }
    }

    #[inline]
    pub(crate) fn native_owned(&self) -> utils_Entity {
        self.identify.native.clone()
    }

    #[inline]
    pub(crate) fn engine(&self) -> Engine {
        self.engine.upgrade_engine().ok().unwrap()
    }

    #[inline]
    pub fn create_camera_component(&mut self) -> Option<&mut Camera> {
        self.delete_camera_component();
        let camera = unsafe {
            Camera::try_from_native(
                self.engine().downgrade(),
                self,
                filament_Engine_createCamera(self.engine().native_mut(), self.native_owned()),
            )
        };
        self.components.camera = camera;
        self.components.camera.as_mut()
    }

    #[inline]
    pub fn get_camera_component(&self) -> Option<&Camera> {
        self.components.camera.as_ref()
    }

    #[inline]
    pub fn get_camera_component_mut(&mut self) -> Option<&mut Camera> {
        self.components.camera.as_mut()
    }

    #[inline]
    pub fn delete_camera_component(&mut self) -> &mut Self {
        if let Some(_) = self.components.camera {
            unsafe {
                filament_Engine_destroyCameraComponent(
                    self.engine().native_mut(),
                    self.native_owned(),
                )
            };
            self.components.camera = None;
        }
        self
    }

    #[inline]
    pub fn create_renderable_component(
        &mut self,
        renderable_options: &mut RenderableOptions,
    ) -> Option<&mut Renderable> {
        self.delete_renderable_component();
        let renderable = Renderable::from_options(self, renderable_options);
        self.components.renderable = renderable;
        self.components.renderable.as_mut()
    }

    #[inline]
    pub fn get_renderable_component(&self) -> Option<&Renderable> {
        self.components.renderable.as_ref()
    }

    #[inline]
    pub fn get_renderable_component_mut(&mut self) -> Option<&mut Renderable> {
        self.components.renderable.as_mut()
    }

    #[inline]
    pub fn delete_renderable_component(&mut self) -> &mut Self {
        if let Some(_) = self.components.renderable {
            Renderable::destroy_instance(self);
            self.components.renderable = None;
        }
        self
    }
}
