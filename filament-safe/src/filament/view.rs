use std::{ffi, ptr, rc::Rc};

use filament_bindings::{
    filament_Engine_createView, filament_Engine_destroy19, filament_MultiSampleAntiAliasingOptions,
    filament_ScreenSpaceReflectionsOptions, filament_TemporalAntiAliasingOptions, filament_View,
    filament_View_getAntiAliasing, filament_View_getBlendMode, filament_View_getName,
    filament_View_getSampleCount, filament_View_getViewport, filament_View_getVisibleLayers,
    filament_View_isScreenSpaceRefractionEnabled, filament_View_setAntiAliasing,
    filament_View_setBlendMode, filament_View_setCamera,
    filament_View_setMultiSampleAntiAliasingOptions, filament_View_setName,
    filament_View_setSampleCount, filament_View_setScene,
    filament_View_setScreenSpaceReflectionsOptions, filament_View_setScreenSpaceRefractionEnabled,
    filament_View_setTemporalAntiAliasingOptions, filament_View_setViewport,
    filament_View_setVisibleLayers, filament_View_setPostProcessingEnabled,
};

use crate::{prelude::NativeHandle, utils::Entity};

use super::{AntiAliasing, BlendMode, Engine, Scene, Viewport};

pub struct View {
    native: Rc<ptr::NonNull<filament_View>>,
    engine: Engine,
    // hold references
    scene: Option<Scene>,
    camera_entity: Option<Entity>,
}

impl NativeHandle<filament_View> for View {
    #[inline]
    fn native(&self) -> *const filament_View {
        self.native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_View {
        self.native.as_ptr()
    }
}

impl View {
    #[inline]
    pub(crate) fn try_from_native(engine: Engine, native: *mut filament_View) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self {
            native: Rc::new(ptr),
            engine,
            scene: None,
            camera_entity: None,
        })
    }

    #[inline]
    pub fn create(engine: &mut Engine) -> Option<Self> {
        unsafe {
            Self::try_from_native(
                engine.clone(),
                filament_Engine_createView(engine.native_mut()),
            )
        }
    }

    #[inline]
    pub fn set_name(&mut self, name: impl AsRef<str>) -> Result<&mut Self, ffi::NulError> {
        let cname = ffi::CString::new(name.as_ref())?;
        unsafe { filament_View_setName(self.native_mut(), cname.as_ptr()) };
        Ok(self)
    }

    #[inline]
    pub fn get_name(&self) -> String {
        unsafe {
            ffi::CStr::from_ptr(filament_View_getName(self.native()))
                .to_string_lossy()
                .to_string()
        }
    }

    #[inline]
    pub fn set_scene(&mut self, scene: &mut Scene) {
        unsafe { filament_View_setScene(self.native_mut(), scene.native_mut()) }
        self.scene = Some(scene.clone())
    }

    #[inline]
    pub fn get_scene(&self) -> Option<&Scene> {
        self.scene.as_ref()
    }

    // TODO: set&get render target

    #[inline]
    pub fn set_viewport(&mut self, viewport: &Viewport) {
        unsafe { filament_View_setViewport(self.native_mut(), viewport.as_native()) }
    }

    #[inline]
    pub fn get_viewport(&self) -> Viewport {
        unsafe { Viewport::new_from_ptr(filament_View_getViewport(self.native())) }
    }

    #[inline]
    pub fn set_camera_entity(&mut self, camera_entity: &mut Entity) {
        let camera = camera_entity
            .get_camera_component_mut()
            .map(|c| c.native_mut())
            .unwrap_or(ptr::null_mut());
        unsafe { filament_View_setCamera(self.native_mut(), camera) };
        self.camera_entity = Some(camera_entity.clone());
    }

    #[inline]
    pub fn unset_camera_entity(&mut self) {
        unsafe { filament_View_setCamera(self.native_mut(), ptr::null_mut()) };
        self.camera_entity = None;
    }

    #[inline]
    pub fn get_camera_entity(&self) -> Option<&Entity> {
        self.camera_entity.as_ref()
    }

    #[inline]
    pub fn set_blend_mode(&mut self, blend_mode: BlendMode) {
        unsafe { filament_View_setBlendMode(self.native_mut(), blend_mode.into()) }
    }

    #[inline]
    pub fn get_blend_mode(&self) -> BlendMode {
        unsafe { BlendMode::from(filament_View_getBlendMode(self.native())) }
    }

    #[inline]
    pub fn set_visible_layers(&mut self, select: u8, values: u8) {
        unsafe { filament_View_setVisibleLayers(self.native_mut(), select, values) }
    }

    #[inline]
    pub fn get_visible_layers(&self) -> u8 {
        unsafe { filament_View_getVisibleLayers(self.native()) }
    }

    #[inline]
    pub fn set_screen_space_refraction_enabled(&mut self, enabled: bool) {
        unsafe { filament_View_setScreenSpaceRefractionEnabled(self.native_mut(), enabled) }
    }

    #[inline]
    pub fn is_screen_space_refraction_enabled(&self) -> bool {
        unsafe { filament_View_isScreenSpaceRefractionEnabled(self.native()) }
    }

    #[inline]
    pub fn set_sample_count(&mut self, count: u8) {
        unsafe { filament_View_setSampleCount(self.native_mut(), count) }
    }

    #[inline]
    pub fn get_sample_count(&self) -> u8 {
        unsafe { filament_View_getSampleCount(self.native()) }
    }

    #[inline]
    pub fn set_anti_aliasing(&mut self, anti_aliasing_type: AntiAliasing) {
        unsafe { filament_View_setAntiAliasing(self.native_mut(), anti_aliasing_type.into()) }
    }

    #[inline]
    pub fn get_anti_aliasing(&self) -> AntiAliasing {
        unsafe { AntiAliasing::from(filament_View_getAntiAliasing(self.native())) }
    }

    #[inline]
    pub fn set_temporal_anti_aliasing_options(
        &mut self,
        options: filament_TemporalAntiAliasingOptions,
    ) {
        unsafe { filament_View_setTemporalAntiAliasingOptions(self.native_mut(), options) }
    }

    // TODO:
    // #[inline]
    // pub fn get_temporal_anti_aliasing_options(&self) -> Option<filament_TemporalAntiAliasingOptions> {
    //     unsafe { (*filament_View_getTemporalAntiAliasingOptions(self.native())).clone() }
    // }

    #[inline]
    pub fn set_screen_space_reflections_options(
        &mut self,
        options: filament_ScreenSpaceReflectionsOptions,
    ) {
        unsafe { filament_View_setScreenSpaceReflectionsOptions(self.native_mut(), options) }
    }

    // TODO:
    // #[inline]
    // pub fn getScreenSpaceReflectionsOptions(&self) -> Option<filament_ScreenSpaceReflectionsOptions> {
    //     unsafe { (*filament_View_getScreenSpaceReflectionsOptions(self.native())).clone() }
    // }

    #[inline]
    pub fn set_multi_sample_anti_aliasing_options(
        &mut self,
        options: filament_MultiSampleAntiAliasingOptions,
    ) {
        unsafe { filament_View_setMultiSampleAntiAliasingOptions(self.native_mut(), options) }
    }

    // TODO:
    // #[inline]
    // pub fn getMultiSampleAntiAliasingOptions(&self) -> Option<filament_MultiSampleAntiAliasingOptions> {
    //     unsafe { (*filament_View_getMultiSampleAntiAliasingOptions(self.native())).clone() }
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
        unsafe { filament_View_setPostProcessingEnabled(self.native_mut(), enabled) }
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

impl Drop for View {
    #[inline]
    fn drop(&mut self) {
        if let Some(_) = Rc::get_mut(&mut self.native) {
            unsafe { filament_Engine_destroy19(self.engine.native_mut(), self.native()) };
        }
    }
}
