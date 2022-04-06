use std::ptr;

use crate::{
    bindgen,
    filament::{Material, VertexAttribute},
};

#[repr(transparent)]
pub struct MaterialProvider {
    native: ptr::NonNull<bindgen::gltfio_MaterialProvider>,
}

impl MaterialProvider {
    #[inline]
    pub fn native(&self) -> *const bindgen::gltfio_MaterialProvider {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::gltfio_MaterialProvider {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::gltfio_MaterialProvider) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(MaterialProvider { native: ptr })
    }

    // Creates a material provider that builds materials on the fly, composing GLSL at run time.
    pub unsafe fn create_material_generator(
        engine: &mut crate::filament::Engine,
        optimize_shaders: bool,
    ) -> Option<Self> {
        let generator = bindgen::helper_gltfio_material_provider_create_material_generator(
            engine.native_mut(),
            optimize_shaders,
        );
        Self::try_from_native(generator)
    }

    // Creates a material provider that loads a small set of pre-built materials.
    pub unsafe fn create_ubershader_loader(engine: &mut crate::filament::Engine) -> Option<Self> {
        let generator =
            bindgen::helper_gltfio_material_provider_create_ubershader_loader(engine.native_mut());
        Self::try_from_native(generator)
    }

    pub unsafe fn get_materials(&self) -> Option<Vec<Material>> {
        let count = bindgen::helper_gltfio_material_provider_get_materials_count(self.native());
        let native_material_ptr =
            bindgen::helper_gltfio_material_provider_get_materials(self.native());
        let native_materials = core::slice::from_raw_parts(native_material_ptr, count);

        let mut result = Vec::with_capacity(native_materials.len());
        for p in native_materials {
            if let Some(material) = Material::try_from_native(*p as *mut _) {
                result.push(material)
            } else {
                return None;
            }
        }

        Some(result)
    }

    #[inline]
    pub unsafe fn destroy_materials(&mut self) {
        bindgen::helper_gltfio_material_provider_destroy_materials(self.native_mut())
    }

    #[inline]
    pub unsafe fn needs_dummy_data(&mut self, attribute: VertexAttribute) -> bool {
        bindgen::helper_gltfio_material_provider_needs_dummy_data(self.native(), attribute.into())
    }
}

impl Drop for MaterialProvider {
    fn drop(&mut self) {
        unsafe { bindgen::helper_gltfio_material_provider_delete(self.native_mut()) }
    }
}
