use std::{ffi, ptr};

use filament_bindings::{
    filament_Engine_destroy9, filament_Material, filament_Material_Builder,
    filament_Material_createInstance, filament_Material_getBlendingMode,
    filament_Material_getCullingMode, filament_Material_getInterpolation,
    filament_Material_getMaskThreshold, filament_Material_getMaterialDomain,
    filament_Material_getName, filament_Material_getParameterCount,
    filament_Material_getReflectionMode, filament_Material_getRefractionMode,
    filament_Material_getRefractionType, filament_Material_getRequiredAttributes,
    filament_Material_getShading, filament_Material_getSpecularAntiAliasingThreshold,
    filament_Material_getSpecularAntiAliasingVariance, filament_Material_getTransparencyMode,
    filament_Material_getVertexDomain, filament_Material_hasParameter,
    filament_Material_hasShadowMultiplier, filament_Material_hasSpecularAntiAliasing,
    filament_Material_isColorWriteEnabled, filament_Material_isDepthCullingEnabled,
    filament_Material_isDepthWriteEnabled, filament_Material_isDoubleSided,
    filament_Material_isSampler, size_t,
};

use crate::{
    backend::CullingMode,
    prelude::{EngineData, EngineDrop, NativeHandle, RcHandle},
};

use super::{
    BlendingMode, Engine, Interpolation, MaterialDomain, MaterialInstance, ReflectionMode,
    RefractionMode, RefractionType, Shading, TransparencyMode, VertexDomain, WeakEngine,
};

pub struct MaterialBuilder {
    native: filament_Material_Builder,
}

impl MaterialBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            native: unsafe { filament_Material_Builder::new() },
        }
    }

    #[inline]
    pub fn from(r: &MaterialBuilder) -> Self {
        Self {
            native: unsafe { filament_Material_Builder::new1(&r.native) },
        }
    }

    #[inline]
    pub fn package<'a>(&mut self, payload: &'a [u8]) -> &mut Self {
        unsafe {
            self.native
                .package(payload.as_ptr() as *const _, payload.len() as size_t)
        };
        self
    }

    #[inline]
    pub fn build(&mut self, engine: &mut Engine) -> Option<Material> {
        Material::try_from_native(engine.downgrade(), unsafe {
            self.native.build(engine.native_mut())
        })
    }
}

impl Drop for MaterialBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.native.destruct() }
    }
}

#[derive(Clone)]
pub struct MaterialData {
    native: ptr::NonNull<filament_Material>,
}

pub type Material = RcHandle<EngineData<MaterialData>>;

impl NativeHandle<filament_Material> for Material {
    #[inline]
    fn native(&self) -> *const filament_Material {
        self.data().native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_Material {
        self.data_mut().native.as_ptr()
    }
}

impl Material {
    #[inline]
    pub(crate) fn try_from_native(
        engine: WeakEngine,
        native: *mut filament_Material,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self::new(EngineData::new(
            MaterialData { native: ptr },
            engine,
        )))
    }
}

impl Material {
    #[inline]
    pub fn create_instance(&self) -> Option<MaterialInstance> {
        unsafe {
            MaterialInstance::try_from_native(
                self.engine().clone(),
                self.clone(),
                filament_Material_createInstance(self.native(), ptr::null()),
            )
        }
    }

    #[inline]
    pub fn create_instance_name(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<MaterialInstance>, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;
        unsafe {
            Ok(MaterialInstance::try_from_native(
                self.engine().clone(),
                self.clone(),
                filament_Material_createInstance(self.native(), c_name.as_ptr()),
            ))
        }
    }

    #[inline]
    pub fn get_name(&self) -> String {
        unsafe {
            ffi::CStr::from_ptr(filament_Material_getName(self.native()))
                .to_string_lossy()
                .to_string()
        }
    }

    #[inline]
    pub fn get_shading(&self) -> Shading {
        unsafe { Shading::from(filament_Material_getShading(self.native())) }
    }
    #[inline]
    pub fn get_interpolation(&self) -> Interpolation {
        unsafe { Interpolation::from(filament_Material_getInterpolation(self.native())) }
    }
    #[inline]
    pub fn get_blending_mode(&self) -> BlendingMode {
        unsafe { BlendingMode::from(filament_Material_getBlendingMode(self.native())) }
    }
    #[inline]
    pub fn get_vertex_domain(&self) -> VertexDomain {
        unsafe { VertexDomain::from(filament_Material_getVertexDomain(self.native())) }
    }
    #[inline]
    pub fn get_material_domain(&self) -> MaterialDomain {
        unsafe { MaterialDomain::from(filament_Material_getMaterialDomain(self.native())) }
    }
    #[inline]
    pub fn get_culling_mode(&self) -> CullingMode {
        unsafe { CullingMode::from(filament_Material_getCullingMode(self.native())) }
    }
    #[inline]
    pub fn get_transparency_mode(&self) -> TransparencyMode {
        unsafe { TransparencyMode::from(filament_Material_getTransparencyMode(self.native())) }
    }
    #[inline]
    pub fn is_color_write_enabled(&self) -> bool {
        unsafe { filament_Material_isColorWriteEnabled(self.native()) }
    }
    #[inline]
    pub fn is_depth_write_enabled(&self) -> bool {
        unsafe { filament_Material_isDepthWriteEnabled(self.native()) }
    }
    #[inline]
    pub fn is_depth_culling_enabled(&self) -> bool {
        unsafe { filament_Material_isDepthCullingEnabled(self.native()) }
    }
    #[inline]
    pub fn is_double_sided(&self) -> bool {
        unsafe { filament_Material_isDoubleSided(self.native()) }
    }
    #[inline]
    pub fn get_mask_threshold(&self) -> f32 {
        unsafe { filament_Material_getMaskThreshold(self.native()) }
    }
    #[inline]
    pub fn has_shadow_multiplier(&self) -> bool {
        unsafe { filament_Material_hasShadowMultiplier(self.native()) }
    }
    #[inline]
    pub fn has_specular_anti_aliasing(&self) -> bool {
        unsafe { filament_Material_hasSpecularAntiAliasing(self.native()) }
    }
    #[inline]
    pub fn get_specular_anti_aliasing_variance(&self) -> f32 {
        unsafe { filament_Material_getSpecularAntiAliasingVariance(self.native()) }
    }
    #[inline]
    pub fn get_specular_anti_aliasing_threshold(&self) -> f32 {
        unsafe { filament_Material_getSpecularAntiAliasingThreshold(self.native()) }
    }
    #[inline]
    pub fn get_required_attributes(&self) -> u32 {
        unsafe { filament_Material_getRequiredAttributes(self.native()) }
    }
    #[inline]
    pub fn get_refraction_mode(&self) -> RefractionMode {
        unsafe { RefractionMode::from(filament_Material_getRefractionMode(self.native())) }
    }
    #[inline]
    pub fn get_refraction_type(&self) -> RefractionType {
        unsafe { RefractionType::from(filament_Material_getRefractionType(self.native())) }
    }
    #[inline]
    pub fn get_reflection_mode(&self) -> ReflectionMode {
        unsafe { ReflectionMode::from(filament_Material_getReflectionMode(self.native())) }
    }
    #[inline]
    pub fn get_parameter_count(&self) -> usize {
        unsafe { filament_Material_getParameterCount(self.native()) as usize }
    }
    // TODO:
    // #[inline]
    // pub fn getParameters(
    //     &self,
    //     parameters: *mut filament_Material_ParameterInfo,
    //     count: usize,
    // ) -> size_t {
    //     filament_Material_getParameters(self, parameters, count)
    // }
    #[inline]
    pub fn has_parameter(&self, name: impl AsRef<str>) -> Result<bool, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;
        unsafe {
            Ok(filament_Material_hasParameter(
                self.native(),
                c_name.as_ptr(),
            ))
        }
    }

    #[inline]
    pub fn is_sampler(&self, name: *const ::std::os::raw::c_char) -> bool {
        unsafe { filament_Material_isSampler(self.native(), name) }
    }
    // TODO: setDefaultParameter
    // TODO:
    // #[inline]
    // pub fn get_default_instance(&mut self) -> Option<MaterialInstance> {
    //     unsafe {
    //         MaterialInstance::try_from_native(
    //             self.engine.clone(),
    //             self.clone(),
    //             filament_Material_getDefaultInstance(self.native_mut()),
    //             true,
    //         )
    //     }
    // }
}

impl EngineDrop for MaterialData {
    #[inline]
    fn drop(&mut self, engine: &mut Engine) {
        unsafe { filament_Engine_destroy9(engine.native_mut(), self.native.as_ptr()) };
    }
}
