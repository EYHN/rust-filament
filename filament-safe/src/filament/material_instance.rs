use std::{ffi, ptr, rc::Rc};

use filament_bindings::{
    filament_Engine_destroy10, filament_MaterialInstance, filament_MaterialInstance_duplicate,
    filament_MaterialInstance_getName, filament_MaterialInstance_setParameter,
    filament_MaterialInstance_setScissor, filament_MaterialInstance_unsetScissor, filament_MaterialInstance_setPolygonOffset, filament_MaterialInstance_setMaskThreshold, filament_MaterialInstance_setSpecularAntiAliasingThreshold, filament_MaterialInstance_setSpecularAntiAliasingVariance, filament_MaterialInstance_setTransparencyMode, filament_MaterialInstance_setDoubleSided, filament_MaterialInstance_setCullingMode, filament_MaterialInstance_setDepthCulling, filament_MaterialInstance_setDepthWrite, filament_MaterialInstance_setColorWrite,
};

use crate::{prelude::NativeHandle, backend::CullingMode};

use super::{Engine, Material, Texture, TextureSampler, TransparencyMode};

#[derive(Clone)]
pub struct MaterialInstance {
    native: Rc<ptr::NonNull<filament_MaterialInstance>>,
    engine: Engine,
    // hold references
    material: Material,
}

impl NativeHandle<filament_MaterialInstance> for MaterialInstance {
    #[inline]
    fn native(&self) -> *const filament_MaterialInstance {
        self.native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_MaterialInstance {
        self.native.as_ptr()
    }
}

impl MaterialInstance {
    #[inline]
    pub(crate) fn try_from_native(
        engine: Engine,
        material: Material,
        native: *mut filament_MaterialInstance,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self {
            native: Rc::new(ptr),
            engine,
            material,
        })
    }

    #[inline]
    pub fn duplicate(other: &MaterialInstance) -> Option<MaterialInstance> {
        unsafe {
            Self::try_from_native(
                other.engine.clone(),
                other.material.clone(),
                filament_MaterialInstance_duplicate(other.native(), ptr::null()),
            )
        }
    }

    #[inline]
    pub fn duplicate_new_name(
        other: &MaterialInstance,
        name: impl AsRef<str>,
    ) -> Result<Option<MaterialInstance>, ffi::NulError> {
        let c_name = ffi::CString::new(name.as_ref())?;
        unsafe {
            Ok(Self::try_from_native(
                other.engine.clone(),
                other.material.clone(),
                filament_MaterialInstance_duplicate(other.native(), c_name.as_ptr()),
            ))
        }
    }

    #[inline]
    pub fn get_material(&self) -> &Material {
        &self.material
    }

    #[inline]
    pub fn get_name(&self) -> String {
        unsafe {
            ffi::CStr::from_ptr(filament_MaterialInstance_getName(self.native()))
                .to_string_lossy()
                .to_string()
        }
    }

    // TODO: setParameter

    #[inline]
    pub fn set_texture_parameter(
        &mut self,
        name: Option<impl AsRef<str>>,
        texture: &Texture,
        sampler: &TextureSampler,
    ) -> Result<&mut Self, ffi::NulError> {
        let c_name = if let Some(name) = name {
            Some(ffi::CString::new(name.as_ref())?)
        } else {
            None
        };

        unsafe {
            filament_MaterialInstance_setParameter(
                self.native_mut(),
                c_name.map(|s| s.as_ptr()).unwrap_or(ptr::null()),
                texture.native(),
                sampler.native(),
            )
        };
        Ok(self)
    }

    // TODO: set_rgb_parameter
    // TODO: set_rgba_parameter

    #[inline]
    pub fn set_scissor(&mut self, left: u32, bottom: u32, width: u32, height: u32) -> &mut Self {
        unsafe { filament_MaterialInstance_setScissor(self.native_mut(), left, bottom, width, height) };
        self
    }

    #[inline]
    pub fn unset_scissor(&mut self) -> &mut Self {
        unsafe { filament_MaterialInstance_unsetScissor(self.native_mut()) };
        self
    }

    #[inline]
    pub fn set_polygon_offset(&mut self, scale: f32, constant: f32) -> &mut Self {
        unsafe { filament_MaterialInstance_setPolygonOffset(self.native_mut(), scale, constant) };
        self
    }

    #[inline]
    pub fn set_specular_anti_aliasing_variance(&mut self, variance: f32) -> &mut Self {
        unsafe { filament_MaterialInstance_setSpecularAntiAliasingVariance(self.native_mut(), variance) };
        self
    }

    #[inline]
    pub fn set_specular_anti_aliasing_threshold(&mut self, threshold: f32) -> &mut Self {
        unsafe { filament_MaterialInstance_setSpecularAntiAliasingThreshold(self.native_mut(), threshold) };
        self
    }

    #[inline]
    pub fn set_double_sided(&mut self, double_sided: bool) -> &mut Self {
        unsafe { filament_MaterialInstance_setDoubleSided(self.native_mut(), double_sided) };
        self
    }

    #[inline]
    pub fn set_transparency_mode(&mut self, mode: TransparencyMode) -> &mut Self {
        unsafe { filament_MaterialInstance_setTransparencyMode(self.native_mut(), mode.into()) };
        self
    }

    #[inline]
    pub fn set_culling_mode(&mut self, culling: CullingMode) -> &mut Self {
        unsafe { filament_MaterialInstance_setCullingMode(self.native_mut(), culling.into()) };
        self
    }

    #[inline]
    pub fn set_color_write(&mut self, enable: bool) -> &mut Self {
        unsafe { filament_MaterialInstance_setColorWrite(self.native_mut(), enable) };
        self
    }

    #[inline]
    pub fn set_depth_write(&mut self, enable: bool) -> &mut Self {
        unsafe { filament_MaterialInstance_setDepthWrite(self.native_mut(), enable) };
        self
    }

    #[inline]
    pub fn set_depth_culling(&mut self, enable: bool) -> &mut Self {
        unsafe { filament_MaterialInstance_setDepthCulling(self.native_mut(), enable) };
        self
    }
}

impl Drop for MaterialInstance {
    #[inline]
    fn drop(&mut self) {
        if let Some(_) = Rc::get_mut(&mut self.native) {
            unsafe { filament_Engine_destroy10(self.engine.native_mut(), self.native()) };
        }
    }
}
