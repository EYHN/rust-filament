use std::{ffi, ptr};

use crate::{backend::CullingMode, bindgen};

use super::{
    BlendingMode, Engine, Interpolation, MaterialDomain, MaterialInstance, ReflectionMode,
    RefractionMode, RefractionType, Shading, TransparencyMode, VertexDomain,
};

pub struct MaterialBuilder {
    native: ptr::NonNull<bindgen::filament_Material_Builder>,
}

impl MaterialBuilder {
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn native(&self) -> *const bindgen::filament_Material_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn native_mut(&mut self) -> *mut bindgen::filament_Material_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn try_from_native(
        native: *mut bindgen::filament_Material_Builder,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(MaterialBuilder { native: ptr })
    }

    #[inline]
    pub unsafe fn new() -> Option<Self> {
        Self::try_from_native(bindgen::helper_filament_material_builder_create())
    }

    #[inline]
    pub unsafe fn package<'a>(&mut self, payload: &'a [u8]) -> &mut Self {
        bindgen::filament_Material_Builder_package(
            self.native_mut(),
            payload.as_ptr() as *const _,
            payload.len(),
        );
        self
    }

    #[inline]
    pub unsafe fn build(&mut self, engine: &mut Engine) -> Option<Material> {
        Material::try_from_native(bindgen::filament_Material_Builder_build(
            self.native_mut(),
            engine.native_mut(),
        ))
    }
}

impl Drop for MaterialBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { bindgen::helper_filament_material_builder_delete(self.native_mut()) }
    }
}

#[derive(Clone)]
pub struct Material {
    native: ptr::NonNull<bindgen::filament_Material>,
}

impl Material {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_Material {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_Material {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_Material) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Material { native: ptr })
    }
}

impl Material {
    #[inline]
    pub unsafe fn create_instance(&self) -> Option<MaterialInstance> {
        MaterialInstance::try_from_native(bindgen::filament_Material_createInstance(
            self.native(),
            ptr::null(),
        ))
    }

    #[inline]
    pub unsafe fn create_instance_name(
        &self,
        name: impl AsRef<str>,
    ) -> Result<Option<MaterialInstance>, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;

        Ok(MaterialInstance::try_from_native(
            bindgen::filament_Material_createInstance(self.native(), c_name.as_ptr()),
        ))
    }

    #[inline]
    pub unsafe fn get_name(&self) -> String {
        ffi::CStr::from_ptr(bindgen::filament_Material_getName(self.native()))
            .to_string_lossy()
            .to_string()
    }

    #[inline]
    pub unsafe fn get_shading(&self) -> Shading {
        Shading::from(bindgen::filament_Material_getShading(self.native()))
    }
    #[inline]
    pub unsafe fn get_interpolation(&self) -> Interpolation {
        Interpolation::from(bindgen::filament_Material_getInterpolation(self.native()))
    }
    #[inline]
    pub unsafe fn get_blending_mode(&self) -> BlendingMode {
        BlendingMode::from(bindgen::filament_Material_getBlendingMode(self.native()))
    }
    #[inline]
    pub unsafe fn get_vertex_domain(&self) -> VertexDomain {
        VertexDomain::from(bindgen::filament_Material_getVertexDomain(self.native()))
    }
    #[inline]
    pub unsafe fn get_material_domain(&self) -> MaterialDomain {
        MaterialDomain::from(bindgen::filament_Material_getMaterialDomain(self.native()))
    }
    #[inline]
    pub unsafe fn get_culling_mode(&self) -> CullingMode {
        CullingMode::from(bindgen::filament_Material_getCullingMode(self.native()))
    }
    #[inline]
    pub unsafe fn get_transparency_mode(&self) -> TransparencyMode {
        TransparencyMode::from(bindgen::filament_Material_getTransparencyMode(
            self.native(),
        ))
    }
    #[inline]
    pub unsafe fn is_color_write_enabled(&self) -> bool {
        bindgen::filament_Material_isColorWriteEnabled(self.native())
    }
    #[inline]
    pub unsafe fn is_depth_write_enabled(&self) -> bool {
        bindgen::filament_Material_isDepthWriteEnabled(self.native())
    }
    #[inline]
    pub unsafe fn is_depth_culling_enabled(&self) -> bool {
        bindgen::filament_Material_isDepthCullingEnabled(self.native())
    }
    #[inline]
    pub unsafe fn is_double_sided(&self) -> bool {
        bindgen::filament_Material_isDoubleSided(self.native())
    }
    #[inline]
    pub unsafe fn get_mask_threshold(&self) -> f32 {
        bindgen::filament_Material_getMaskThreshold(self.native())
    }
    #[inline]
    pub unsafe fn has_shadow_multiplier(&self) -> bool {
        bindgen::filament_Material_hasShadowMultiplier(self.native())
    }
    #[inline]
    pub unsafe fn has_specular_anti_aliasing(&self) -> bool {
        bindgen::filament_Material_hasSpecularAntiAliasing(self.native())
    }
    #[inline]
    pub unsafe fn get_specular_anti_aliasing_variance(&self) -> f32 {
        bindgen::filament_Material_getSpecularAntiAliasingVariance(self.native())
    }
    #[inline]
    pub unsafe fn get_specular_anti_aliasing_threshold(&self) -> f32 {
        bindgen::filament_Material_getSpecularAntiAliasingThreshold(self.native())
    }
    #[inline]
    pub unsafe fn get_required_attributes(&self) -> u32 {
        bindgen::filament_Material_getRequiredAttributes(self.native())
    }
    #[inline]
    pub unsafe fn get_refraction_mode(&self) -> RefractionMode {
        RefractionMode::from(bindgen::filament_Material_getRefractionMode(self.native()))
    }
    #[inline]
    pub unsafe fn get_refraction_type(&self) -> RefractionType {
        RefractionType::from(bindgen::filament_Material_getRefractionType(self.native()))
    }
    #[inline]
    pub unsafe fn get_reflection_mode(&self) -> ReflectionMode {
        ReflectionMode::from(bindgen::filament_Material_getReflectionMode(self.native()))
    }

    #[inline]
    pub unsafe fn get_parameter_count(&self) -> usize {
        bindgen::filament_Material_getParameterCount(self.native())
    }
    // TODO:
    // #[inline]
    // pub unsafe fn getParameters(
    //     &self,
    //     parameters: *mut bindgen::filament_Material_ParameterInfo,
    //     count: usize,
    // ) -> size_t {
    //     bindgen::filament_Material_getParameters(self, parameters, count)
    // }
    #[inline]
    pub unsafe fn has_parameter(&self, name: impl AsRef<str>) -> Result<bool, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;

        Ok(bindgen::filament_Material_hasParameter(
            self.native(),
            c_name.as_ptr(),
        ))
    }

    #[inline]
    pub unsafe fn is_sampler(&self, name: *const ::std::os::raw::c_char) -> bool {
        bindgen::filament_Material_isSampler(self.native(), name)
    }

    // TODO: setDefaultParameter
    #[inline]
    pub unsafe fn get_default_instance(&mut self) -> Option<MaterialInstance> {
        MaterialInstance::try_from_native(bindgen::filament_Material_getDefaultInstance(
            self.native_mut(),
        ))
    }
}
