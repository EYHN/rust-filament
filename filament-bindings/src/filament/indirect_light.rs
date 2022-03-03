use std::ptr;

use crate::{
    bindgen,
    math::{Float3, Mat3f},
};

use super::{Engine, Texture};

pub struct IndirectLightBuilder {
    native: ptr::NonNull<bindgen::filament_IndirectLight_Builder>,
}

impl IndirectLightBuilder {
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn native(&self) -> *const bindgen::filament_IndirectLight_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn native_mut(&mut self) -> *mut bindgen::filament_IndirectLight_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn try_from_native(
        native: *mut bindgen::filament_IndirectLight_Builder,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(IndirectLightBuilder { native: ptr })
    }

    #[inline]
    pub unsafe fn new() -> Option<Self> {
        Self::try_from_native(bindgen::helper_filament_indirect_light_builder_create())
    }

    #[inline]
    pub unsafe fn reflections(&mut self, cubemap: &Texture) -> &mut Self {
        bindgen::filament_IndirectLight_Builder_reflections(self.native_mut(), cubemap.native());
        self
    }
    #[inline]
    pub unsafe fn irradiance(&mut self, bands: u8, sh: &Float3) -> &mut Self {
        bindgen::filament_IndirectLight_Builder_irradiance(
            self.native_mut(),
            bands,
            sh.native_ptr(),
        );
        self
    }
    #[inline]
    pub unsafe fn radiance(&mut self, bands: u8, sh: &Float3) -> &mut Self {
        bindgen::filament_IndirectLight_Builder_radiance(self.native_mut(), bands, sh.native_ptr());
        self
    }
    #[inline]
    pub unsafe fn irradiance_cubemap(&mut self, cubemap: &Texture) -> &mut Self {
        bindgen::filament_IndirectLight_Builder_irradiance1(self.native_mut(), cubemap.native());
        self
    }
    #[inline]
    pub unsafe fn intensity(&mut self, env_intensity: f32) -> &mut Self {
        bindgen::filament_IndirectLight_Builder_intensity(self.native_mut(), env_intensity);
        self
    }
    #[inline]
    pub unsafe fn rotation(&mut self, rotation: &Mat3f) -> &mut Self {
        bindgen::filament_IndirectLight_Builder_rotation(self.native_mut(), rotation.native_ptr());
        self
    }

    #[inline]
    pub unsafe fn build(&mut self, engine: &mut Engine) -> Option<IndirectLight> {
        IndirectLight::try_from_native(bindgen::filament_IndirectLight_Builder_build(
            self.native_mut(),
            engine.native_mut(),
        ))
    }
}

impl Drop for IndirectLightBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { bindgen::helper_filament_indirect_light_builder_delete(self.native_mut()) }
    }
}

pub struct IndirectLight {
    native: ptr::NonNull<bindgen::filament_IndirectLight>,
}

impl IndirectLight {
    #[inline]
    pub unsafe fn native(&self) -> *const bindgen::filament_IndirectLight {
        self.native.as_ptr()
    }

    #[inline]
    pub unsafe fn native_mut(&mut self) -> *mut bindgen::filament_IndirectLight {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_IndirectLight) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }

    #[inline]
    pub unsafe fn set_intensity(&mut self, intensity: f32) {
        bindgen::filament_IndirectLight_setIntensity(self.native_mut(), intensity)
    }
    #[inline]
    pub unsafe fn get_intensity(&self) -> f32 {
        bindgen::filament_IndirectLight_getIntensity(self.native())
    }
    #[inline]
    pub unsafe fn set_rotation(&mut self, rotation: &Mat3f) {
        bindgen::filament_IndirectLight_setRotation(self.native_mut(), rotation.native_ptr())
    }
    #[inline]
    pub unsafe fn get_rotation(&self) -> Mat3f {
        Mat3f::from_native(bindgen::filament_IndirectLight_getRotation(self.native()).read())
    }
    #[inline]
    pub unsafe fn get_reflections_texture(&self) -> Option<Texture> {
        Texture::try_from_native(bindgen::filament_IndirectLight_getReflectionsTexture(
            self.native(),
        ) as *mut _)
    }
    #[inline]
    pub unsafe fn get_irradiance_texture(&self) -> Option<Texture> {
        Texture::try_from_native(
            bindgen::filament_IndirectLight_getIrradianceTexture(self.native()) as *mut _,
        )
    }
    #[inline]
    pub unsafe fn get_direction_estimate(sh: &Float3) -> Float3 {
        Float3::from_native(bindgen::filament_IndirectLight_getDirectionEstimate(
            sh.native_ptr(),
        ))
    }
    #[inline]
    pub unsafe fn get_color_estimate(
        sh: &Float3,
        direction: Float3,
    ) -> bindgen::filament_math_float4 {
        bindgen::filament_IndirectLight_getColorEstimate(sh.native_ptr(), direction.native_owned())
    }
    #[inline]
    pub unsafe fn get_direction_estimate1(&self) -> Float3 {
        Float3::from_native(bindgen::filament_IndirectLight_getDirectionEstimate1(
            self.native(),
        ))
    }
    #[inline]
    pub unsafe fn get_color_estimate_direction(&self, direction: Float3) -> bindgen::filament_math_float4 {
        bindgen::filament_IndirectLight_getColorEstimate1(self.native(), direction.native_owned())
    }
}
