use std::{ffi::CString, ptr};

use crate::{
    bindgen,
    filament::{Engine, Material},
    utils::EntityManager,
};

use super::{GltfAsset, MaterialProvider, TextureProvider};

pub struct AssetConfiguration<'a> {
    pub engine: &'a mut Engine,
    pub materials: MaterialProvider,
    pub entities: Option<&'a mut EntityManager>,
    pub default_node_name: Option<String>, // TODO: names: NameComponentManager
}

pub struct AssetLoader {
    native: ptr::NonNull<bindgen::filament_gltfio_AssetLoader>,
    materials: MaterialProvider,
}

impl AssetLoader {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_gltfio_AssetLoader {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_gltfio_AssetLoader {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(
        native: *mut bindgen::filament_gltfio_AssetLoader,
        materials: MaterialProvider,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(AssetLoader {
            native: ptr,
            materials,
        })
    }

    #[inline]
    pub unsafe fn create(mut config: AssetConfiguration) -> Option<Self> {
        let native_default_config_name = if let Some(default_node_name) = config.default_node_name {
            CString::new(default_node_name).ok()
        } else {
            None
        };

        let native_config = bindgen::filament_gltfio_AssetConfiguration {
            engine: config.engine.native_mut(),
            materials: config.materials.native_mut(),
            entities: config
                .entities
                .map(|e| e.native_mut())
                .unwrap_or(core::ptr::null_mut()),
            names: core::ptr::null_mut(),
            defaultNodeName: native_default_config_name
                .map(|cs| cs.as_ptr() as *mut _)
                .unwrap_or(core::ptr::null_mut()),
        };
        Self::try_from_native(
            bindgen::filament_gltfio_AssetLoader_create(&native_config),
            config.materials,
        )
    }

    #[inline]
    pub unsafe fn create_asset(&mut self, bytes: &[u8]) -> Option<GltfAsset> {
        GltfAsset::try_from_native(bindgen::filament_gltfio_AssetLoader_createAsset(
            self.native_mut(),
            bytes.as_ptr(),
            bytes.len() as u32,
        ))
    }

    // TODO: createInstancedAsset
    // TODO: createInstance

    #[inline]
    pub unsafe fn enable_diagnostics(&mut self, enable: bool) {
        bindgen::filament_gltfio_AssetLoader_enableDiagnostics(self.native_mut(), enable)
    }

    #[inline]
    pub unsafe fn destroy_asset(&mut self, asset: GltfAsset) {
        bindgen::filament_gltfio_AssetLoader_destroyAsset(self.native_mut(), asset.native())
    }

    #[inline]
    pub unsafe fn get_materials(&self) -> Option<Vec<Material>> {
        self.materials.get_materials()
    }

    #[inline]
    pub unsafe fn destroy_materials(&mut self) {
        self.materials.destroy_materials()
    }

    #[inline]
    pub unsafe fn get_material_provider(&mut self) -> &MaterialProvider {
        &self.materials
    }

    #[inline]
    pub unsafe fn get_material_provider_mut(&mut self) -> &mut MaterialProvider {
        &mut self.materials
    }

    // TODO: getNames
}

impl Drop for AssetLoader {
    fn drop(&mut self) {
        unsafe { bindgen::filament_gltfio_AssetLoader_destroy(&mut self.native_mut()) }
    }
}
