use std::ptr;

use crate::{bindgen, math::Float4};

use super::{Engine, Texture};

pub struct SkyboxBuilder {
    native: ptr::NonNull<bindgen::filament_Skybox_Builder>,
}

impl SkyboxBuilder {
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn native(&self) -> *const bindgen::filament_Skybox_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn native_mut(&mut self) -> *mut bindgen::filament_Skybox_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn try_from_native(
        native: *mut bindgen::filament_Skybox_Builder,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(SkyboxBuilder { native: ptr })
    }

    #[inline]
    pub unsafe fn new() -> Option<Self> {
        Self::try_from_native(bindgen::helper_filament_skybox_builder_create())
    }

    #[inline]
    pub unsafe fn environment(&mut self, cubemap: &mut Texture) -> &mut Self {
        bindgen::filament_Skybox_Builder_environment(self.native_mut(), cubemap.native_mut());
        self
    }
    #[inline]
    pub unsafe fn show_sun(&mut self, show: bool) -> &mut Self {
        bindgen::filament_Skybox_Builder_showSun(self.native_mut(), show);
        self
    }
    #[inline]
    pub unsafe fn intensity(&mut self, env_intensity: f32) -> &mut Self {
        bindgen::filament_Skybox_Builder_intensity(self.native_mut(), env_intensity);
        self
    }
    #[inline]
    pub unsafe fn color(&mut self, color: Float4) -> &mut Self {
        bindgen::filament_Skybox_Builder_color(self.native_mut(), color.native_owned());
        self
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: &mut Engine) -> Option<Skybox> {
        Skybox::try_from_native(bindgen::filament_Skybox_Builder_build(
            self.native_mut(),
            engine.native_mut(),
        ))
    }
}

impl Drop for SkyboxBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { bindgen::helper_filament_skybox_builder_delete(self.native_mut()) }
    }
}

pub struct Skybox {
    native: ptr::NonNull<bindgen::filament_Skybox>,
}

impl Skybox {
    #[inline]
    pub unsafe fn native(&self) -> *const bindgen::filament_Skybox {
        self.native.as_ptr()
    }

    #[inline]
    pub unsafe fn native_mut(&mut self) -> *mut bindgen::filament_Skybox {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_Skybox) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }

    #[inline]
    pub unsafe fn set_color(&mut self, color: Float4) {
        bindgen::filament_Skybox_setColor(self.native_mut(), color.native_owned())
    }
    #[inline]
    pub unsafe fn set_layer_mask(&mut self, select: u8, values: u8) {
        bindgen::filament_Skybox_setLayerMask(self.native_mut(), select, values)
    }
    #[inline]
    pub unsafe fn get_layer_mask(&self) -> u8 {
        bindgen::filament_Skybox_getLayerMask(self.native())
    }
    #[inline]
    pub unsafe fn get_intensity(&self) -> f32 {
        bindgen::filament_Skybox_getIntensity(self.native())
    }
    #[inline]
    pub unsafe fn get_texture(&self) -> Option<Texture> {
        Texture::try_from_native(bindgen::filament_Skybox_getTexture(self.native()) as *mut _)
    }
}
