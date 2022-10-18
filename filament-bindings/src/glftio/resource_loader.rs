use std::{ffi::CString, ptr};

use crate::{backend::BufferDescriptor, bindgen, filament::Engine};

use super::{GltfAsset, TextureProvider};

pub struct ResourceConfiguration<'a> {
    pub engine: &'a mut Engine,

    pub gltf_path: Option<String>,

    pub normalize_skinning_weights: bool,
}

#[repr(transparent)]
pub struct ResourceLoader {
    native: ptr::NonNull<bindgen::filament_gltfio_ResourceLoader>,
}

impl ResourceLoader {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_gltfio_ResourceLoader {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_gltfio_ResourceLoader {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_gltfio_ResourceLoader) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(ResourceLoader { native: ptr })
    }

    pub unsafe fn create(config: ResourceConfiguration) -> Option<Self> {
        let native_gltf_path = if let Some(gltf_path) = config.gltf_path {
            CString::new(gltf_path).ok()
        } else {
            None
        };

        let native_config = bindgen::filament_gltfio_ResourceConfiguration {
            engine: config.engine.native_mut(),
            gltfPath: native_gltf_path
                .as_ref()
                .map(|s| s.as_ptr() as *mut _)
                .unwrap_or(core::ptr::null_mut()),
            normalizeSkinningWeights: config.normalize_skinning_weights,
        };

        ResourceLoader::try_from_native(bindgen::helper_filament_gltfio_resource_loader_create(
            &native_config,
        ))
    }

    #[inline]
    pub unsafe fn add_resource_data(
        &mut self,
        uri: &str,
        buffer: BufferDescriptor<u8>,
    ) -> Result<(), std::ffi::NulError> {
        let c_uri = CString::new(uri)?;

        bindgen::filament_gltfio_ResourceLoader_addResourceData(
            self.native_mut(),
            c_uri.as_ptr(),
            &mut buffer.into_native(),
        );

        Ok(())
    }

    #[inline]
    pub unsafe fn has_resource_data(&self, uri: &str) -> Result<bool, std::ffi::NulError> {
        let c_uri = CString::new(uri)?;

        Ok(bindgen::filament_gltfio_ResourceLoader_hasResourceData(
            self.native(),
            c_uri.as_ptr(),
        ))
    }

    #[inline]
    pub unsafe fn evict_resource_data(&mut self) {
        bindgen::filament_gltfio_ResourceLoader_evictResourceData(self.native_mut())
    }

    #[inline]
    pub unsafe fn load_resources(&mut self, asset: &mut GltfAsset) -> bool {
        bindgen::filament_gltfio_ResourceLoader_loadResources(self.native_mut(), asset.native_mut())
    }

    // TODO: asyncBeginLoad
    // TODO: asyncGetLoadProgress
    // TODO: asyncUpdateLoad
    // TODO: asyncCancelLoad

    #[inline]
    pub unsafe fn add_texture_provider(
        &mut self,
        mime_type: &str,
        provider: &mut TextureProvider,
    ) -> Result<(), std::ffi::NulError> {
        let c_mime_type = CString::new(mime_type)?;
        Ok(
            bindgen::helper_filament_gltfio_resource_loader_add_texture_provider(
                self.native_mut(),
                c_mime_type.as_ptr(),
                provider.native_mut(),
            ),
        )
    }
}

impl Drop for ResourceLoader {
    fn drop(&mut self) {
        unsafe { bindgen::helper_filament_gltfio_resource_loader_delete(self.native_mut()) }
    }
}
