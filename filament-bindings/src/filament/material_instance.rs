use std::{ffi, ptr};

use crate::{backend::CullingMode, bindgen, math::{Float3, self, Float4}};

use super::{Material, RgbType, RgbaType, Texture, TextureSampler, TransparencyMode};

macro_rules! impl_set_parameter_method {
    ($t:ident, $helper:ident, $value_type:ty) => {
        #[inline]
        pub unsafe fn $t(
            &mut self,
            name: impl AsRef<str>,
            value: &$value_type,
        ) -> Result<&mut Self, ffi::NulError> {
            let c_name = ffi::CString::new(name.as_ref())?;
            bindgen::$helper(self.native_mut(), c_name.as_ptr(), value);
            Ok(self)
        }
    };
}

macro_rules! impl_set_wrapped_parameter_method {
    ($t:ident, $helper:ident, $value_type:ty) => {
        #[inline]
        pub unsafe fn $t(
            &mut self,
            name: impl AsRef<str>,
            value: &$value_type,
        ) -> Result<&mut Self, ffi::NulError> {
            let c_name = ffi::CString::new(name.as_ref())?;
            bindgen::$helper(self.native_mut(), c_name.as_ptr(), value.native_ptr());
            Ok(self)
        }
    };
}

#[repr(transparent)]
pub struct MaterialInstance {
    native: ptr::NonNull<bindgen::filament_MaterialInstance>,
}

impl MaterialInstance {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_MaterialInstance {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_MaterialInstance {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_MaterialInstance) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(MaterialInstance { native: ptr })
    }

    #[inline]
    pub unsafe fn duplicate(other: &MaterialInstance) -> Option<MaterialInstance> {
        Self::try_from_native(bindgen::filament_MaterialInstance_duplicate(
            other.native(),
            ptr::null(),
        ))
    }

    #[inline]
    pub unsafe fn duplicate_new_name(
        other: &MaterialInstance,
        name: impl AsRef<str>,
    ) -> Result<Option<MaterialInstance>, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;
        Ok(Self::try_from_native(
            bindgen::filament_MaterialInstance_duplicate(other.native(), c_name.as_ptr()),
        ))
    }

    #[inline]
    pub unsafe fn get_material(&self) -> Option<Material> {
        Material::try_from_native(
            bindgen::filament_MaterialInstance_getMaterial(self.native()) as *mut _,
        )
    }

    #[inline]
    pub unsafe fn get_name(&self) -> String {
        ffi::CStr::from_ptr(bindgen::filament_MaterialInstance_getName(self.native()))
            .to_string_lossy()
            .to_string()
    }

    impl_set_parameter_method!(
        set_float_parameter,
        helper_material_instance_setParameter_float,
        f32
    );
    impl_set_parameter_method!(
        set_int32_parameter,
        helper_material_instance_setParameter_int32,
        i32
    );
    impl_set_parameter_method!(
        set_uint32_parameter,
        helper_material_instance_setParameter_uint32,
        u32
    );
    impl_set_wrapped_parameter_method!(
        set_int2_parameter,
        helper_material_instance_setParameter_int2,
        math::Int2
    );
    impl_set_wrapped_parameter_method!(
        set_int3_parameter,
        helper_material_instance_setParameter_int3,
        math::Int3
    );
    impl_set_wrapped_parameter_method!(
        set_int4_parameter,
        helper_material_instance_setParameter_int4,
        math::Int4
    );
    impl_set_wrapped_parameter_method!(
        set_uint2_parameter,
        helper_material_instance_setParameter_uint2,
        math::Uint2
    );
    impl_set_wrapped_parameter_method!(
        set_uint3_parameter,
        helper_material_instance_setParameter_uint3,
        math::Uint3
    );
    impl_set_wrapped_parameter_method!(
        set_uint4_parameter,
        helper_material_instance_setParameter_uint4,
        math::Uint4
    );
    impl_set_wrapped_parameter_method!(
        set_float2_parameter,
        helper_material_instance_setParameter_float2,
        math::Float2
    );
    impl_set_wrapped_parameter_method!(
        set_float3_parameter,
        helper_material_instance_setParameter_float3,
        math::Float3
    );
    impl_set_wrapped_parameter_method!(
        set_float4_parameter,
        helper_material_instance_setParameter_float4,
        math::Float4
    );
    impl_set_wrapped_parameter_method!(
        set_mat4f_parameter,
        helper_material_instance_setParameter_mat4f,
        math::Mat4f
    );
    impl_set_parameter_method!(
        set_bool_parameter,
        helper_material_instance_setParameter_bool,
        bool
    );
    impl_set_wrapped_parameter_method!(
        set_bool2_parameter,
        helper_material_instance_setParameter_bool2,
        math::Bool2
    );
    impl_set_wrapped_parameter_method!(
        set_bool3_parameter,
        helper_material_instance_setParameter_bool3,
        math::Bool3
    );
    impl_set_wrapped_parameter_method!(
        set_bool4_parameter,
        helper_material_instance_setParameter_bool4,
        math::Bool4
    );
    impl_set_wrapped_parameter_method!(
        set_mat3f_parameter,
        helper_material_instance_setParameter_mat3f,
        math::Mat3f
    );

    #[inline]
    pub unsafe fn set_texture_parameter(
        &mut self,
        name: impl AsRef<str>,
        texture: &Texture,
        sampler: &TextureSampler,
    ) -> Result<&mut Self, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;

        bindgen::filament_MaterialInstance_setParameter(
            self.native_mut(),
            c_name.as_ptr(),
            texture.native(),
            sampler.native(),
        );
        Ok(self)
    }

    #[inline]
    pub unsafe fn set_rgb_parameter(
        &mut self,
        name: impl AsRef<str>,
        rgb_type: RgbType,
        value: Float3,
    ) -> Result<&mut Self, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;
        bindgen::filament_MaterialInstance_setParameter1(
            self.native_mut(),
            c_name.as_ptr(),
            rgb_type.into(),
            value.native_owned(),
        );
        Ok(self)
    }

    #[inline]
    pub unsafe fn set_rgba_parameter(
        &mut self,
        name: impl AsRef<str>,
        rgb_type: RgbaType,
        value: Float4,
    ) -> Result<&mut Self, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;
        bindgen::filament_MaterialInstance_setParameter2(
            self.native_mut(),
            c_name.as_ptr(),
            rgb_type.into(),
            value.native_owned(),
        );
        Ok(self)
    }

    #[inline]
    pub unsafe fn set_scissor(
        &mut self,
        left: u32,
        bottom: u32,
        width: u32,
        height: u32,
    ) -> &mut Self {
        bindgen::filament_MaterialInstance_setScissor(
            self.native_mut(),
            left,
            bottom,
            width,
            height,
        );
        self
    }

    #[inline]
    pub unsafe fn unset_scissor(&mut self) -> &mut Self {
        bindgen::filament_MaterialInstance_unsetScissor(self.native_mut());
        self
    }

    #[inline]
    pub unsafe fn set_polygon_offset(&mut self, scale: f32, constant: f32) -> &mut Self {
        bindgen::filament_MaterialInstance_setPolygonOffset(self.native_mut(), scale, constant);
        self
    }

    #[inline]
    pub unsafe fn set_specular_anti_aliasing_variance(&mut self, variance: f32) -> &mut Self {
        bindgen::filament_MaterialInstance_setSpecularAntiAliasingVariance(
            self.native_mut(),
            variance,
        );
        self
    }

    #[inline]
    pub unsafe fn set_specular_anti_aliasing_threshold(&mut self, threshold: f32) -> &mut Self {
        bindgen::filament_MaterialInstance_setSpecularAntiAliasingThreshold(
            self.native_mut(),
            threshold,
        );
        self
    }

    #[inline]
    pub unsafe fn set_double_sided(&mut self, double_sided: bool) -> &mut Self {
        bindgen::filament_MaterialInstance_setDoubleSided(self.native_mut(), double_sided);
        self
    }

    #[inline]
    pub unsafe fn set_transparency_mode(&mut self, mode: TransparencyMode) -> &mut Self {
        bindgen::filament_MaterialInstance_setTransparencyMode(self.native_mut(), mode.into());
        self
    }

    #[inline]
    pub unsafe fn set_culling_mode(&mut self, culling: CullingMode) -> &mut Self {
        bindgen::filament_MaterialInstance_setCullingMode(self.native_mut(), culling.into());
        self
    }

    #[inline]
    pub unsafe fn set_color_write(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_MaterialInstance_setColorWrite(self.native_mut(), enable);
        self
    }

    #[inline]
    pub unsafe fn set_depth_write(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_MaterialInstance_setDepthWrite(self.native_mut(), enable);
        self
    }

    #[inline]
    pub unsafe fn set_depth_culling(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_MaterialInstance_setDepthCulling(self.native_mut(), enable);
        self
    }
}
