use std::{ffi, ptr};

use crate::bindgen;

use super::{AntiAliasing, BlendMode, Camera, Scene, Viewport};

pub struct View {
    native: ptr::NonNull<bindgen::filament_View>,
}

impl View {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_View {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_View {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_View) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }

    #[inline]
    pub fn set_name(&mut self, name: impl AsRef<str>) -> Result<&mut Self, ffi::NulError> {
        let cname = ffi::CString::new(name.as_ref())?;
        unsafe { bindgen::filament_View_setName(self.native_mut(), cname.as_ptr()) };
        Ok(self)
    }

    #[inline]
    pub fn get_name(&self) -> String {
        unsafe {
            ffi::CStr::from_ptr(bindgen::filament_View_getName(self.native()))
                .to_string_lossy()
                .to_string()
        }
    }

    #[inline]
    pub fn set_scene(&mut self, scene: &mut Scene) {
        unsafe { bindgen::filament_View_setScene(self.native_mut(), scene.native_mut()) }
    }

    #[inline]
    pub fn get_scene(&mut self) -> Option<Scene> {
        unsafe { Scene::try_from_native(bindgen::filament_View_getScene(self.native_mut())) }
    }

    // TODO: set&get render target

    #[inline]
    pub fn set_viewport(&mut self, viewport: &Viewport) {
        unsafe { bindgen::filament_View_setViewport(self.native_mut(), viewport.as_native()) }
    }

    #[inline]
    pub fn get_viewport(&self) -> Viewport {
        unsafe { Viewport::new_from_ptr(bindgen::filament_View_getViewport(self.native())) }
    }

    #[inline]
    pub fn set_camera(&mut self, camera: &mut Camera) {
        unsafe { bindgen::filament_View_setCamera(self.native_mut(), camera.native_mut()) };
    }

    #[inline]
    pub fn unset_camera(&mut self) {
        unsafe { bindgen::filament_View_setCamera(self.native_mut(), ptr::null_mut()) };
    }

    #[inline]
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        unsafe { bindgen::filament_View_setBlendMode(self.native_mut(), blend_mode.into()) }
    }

    #[inline]
    pub fn get_blend_mode(&self) -> BlendMode {
        unsafe { BlendMode::from(bindgen::filament_View_getBlendMode(self.native())) }
    }

    #[inline]
    pub fn set_visible_layers(&mut self, select: u8, values: u8) {
        unsafe { bindgen::filament_View_setVisibleLayers(self.native_mut(), select, values) }
    }

    #[inline]
    pub fn get_visible_layers(&self) -> u8 {
        unsafe { bindgen::filament_View_getVisibleLayers(self.native()) }
    }

    #[inline]
    pub fn set_screen_space_refraction_enabled(&mut self, enabled: bool) {
        unsafe {
            bindgen::filament_View_setScreenSpaceRefractionEnabled(self.native_mut(), enabled)
        }
    }

    #[inline]
    pub fn is_screen_space_refraction_enabled(&self) -> bool {
        unsafe { bindgen::filament_View_isScreenSpaceRefractionEnabled(self.native()) }
    }

    #[inline]
    pub fn set_sample_count(&mut self, count: u8) {
        unsafe { bindgen::filament_View_setSampleCount(self.native_mut(), count) }
    }

    #[inline]
    pub fn get_sample_count(&self) -> u8 {
        unsafe { bindgen::filament_View_getSampleCount(self.native()) }
    }

    #[inline]
    pub fn set_anti_aliasing(&mut self, anti_aliasing_type: AntiAliasing) {
        unsafe {
            bindgen::filament_View_setAntiAliasing(self.native_mut(), anti_aliasing_type.into())
        }
    }

    #[inline]
    pub fn get_anti_aliasing(&self) -> AntiAliasing {
        unsafe { AntiAliasing::from(bindgen::filament_View_getAntiAliasing(self.native())) }
    }

    #[inline]
    pub fn set_temporal_anti_aliasing_options(
        &mut self,
        options: bindgen::filament_TemporalAntiAliasingOptions,
    ) {
        unsafe { bindgen::filament_View_setTemporalAntiAliasingOptions(self.native_mut(), options) }
    }

    // TODO:
    // #[inline]
    // pub fn get_temporal_anti_aliasing_options(&self) -> Option<bindgen::filament_TemporalAntiAliasingOptions> {
    //     unsafe { (*bindgen::filament_View_getTemporalAntiAliasingOptions(self.native())).clone() }
    // }

    #[inline]
    pub fn set_screen_space_reflections_options(
        &mut self,
        options: bindgen::filament_ScreenSpaceReflectionsOptions,
    ) {
        unsafe {
            bindgen::filament_View_setScreenSpaceReflectionsOptions(self.native_mut(), options)
        }
    }

    // TODO:
    // #[inline]
    // pub fn getScreenSpaceReflectionsOptions(&self) -> Option<bindgen::filament_ScreenSpaceReflectionsOptions> {
    //     unsafe { (*bindgen::filament_View_getScreenSpaceReflectionsOptions(self.native())).clone() }
    // }

    #[inline]
    pub fn set_multi_sample_anti_aliasing_options(
        &mut self,
        options: bindgen::filament_MultiSampleAntiAliasingOptions,
    ) {
        unsafe {
            bindgen::filament_View_setMultiSampleAntiAliasingOptions(self.native_mut(), options)
        }
    }

    // TODO:
    // #[inline]
    // pub fn getMultiSampleAntiAliasingOptions(&self) -> Option<bindgen::filament_MultiSampleAntiAliasingOptions> {
    //     unsafe { (*bindgen::filament_View_getMultiSampleAntiAliasingOptions(self.native())).clone() }
    // }

    // TODO:
    // setColorGrading
    // getColorGrading
    // setAmbientOcclusionOptions
    // getAmbientOcclusionOptions
    // setBloomOptions
    // getBloomOptions
    // setFogOptions
    // getFogOptions
    // setDepthOfFieldOptions
    // getDepthOfFieldOptions
    // setVignetteOptions
    // getVignetteOptions
    // setDithering
    // getDithering
    // setDynamicResolutionOptions
    // getDynamicResolutionOptions
    // setRenderQuality
    // getRenderQuality
    // setDynamicLightingOptions
    // setShadowType
    // setVsmShadowOptions
    // getVsmShadowOptions
    // setSoftShadowOptions
    // getSoftShadowOptions
    #[inline]
    pub fn set_post_processing_enabled(&mut self, enabled: bool) {
        unsafe { bindgen::filament_View_setPostProcessingEnabled(self.native_mut(), enabled) }
    }
    // isPostProcessingEnabled
    // setFrontFaceWindingInverted
    // isFrontFaceWindingInverted
    // setFrustumCullingEnabled
    // isFrustumCullingEnabled
    // setDebugCamera
    // getDirectionalLightCamera
    // pick
    // setAmbientOcclusion
    // getAmbientOcclusion
}
