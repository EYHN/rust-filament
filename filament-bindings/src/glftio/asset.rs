use core::slice;
use std::{ffi::CStr, ptr, str::Utf8Error};

use crate::{bindgen, filament::Aabb, utils::Entity};

#[repr(transparent)]
pub struct GltfAsset {
    native: ptr::NonNull<bindgen::filament_gltfio_FilamentAsset>,
}

impl GltfAsset {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_gltfio_FilamentAsset {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_gltfio_FilamentAsset {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_gltfio_FilamentAsset) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(GltfAsset { native: ptr })
    }

    pub unsafe fn get_entities(&self) -> &[Entity] {
        let ptr = bindgen::filament_gltfio_FilamentAsset_getEntities(self.native());
        let len = bindgen::filament_gltfio_FilamentAsset_getEntityCount(self.native());

        slice::from_raw_parts(ptr as *const _, len)
    }

    pub unsafe fn get_light_entities(&self) -> &[Entity] {
        let ptr = bindgen::filament_gltfio_FilamentAsset_getLightEntities(self.native());
        let len = bindgen::filament_gltfio_FilamentAsset_getLightEntityCount(self.native());

        slice::from_raw_parts(ptr as *const _, len)
    }

    pub unsafe fn get_camera_entities(&self) -> &[Entity] {
        let ptr = bindgen::filament_gltfio_FilamentAsset_getCameraEntities(self.native());
        let len = bindgen::filament_gltfio_FilamentAsset_getCameraEntityCount(self.native());

        slice::from_raw_parts(ptr as *const _, len)
    }

    pub unsafe fn get_root(&self) -> Entity {
        let mut result = Entity::dangling();
        bindgen::helper_filament_gltfio_filament_asset_get_root(
            self.native(),
            result.native_ptr_mut(),
        );

        result
    }

    pub unsafe fn pop_renderables(&mut self) -> Vec<Entity> {
        let len = bindgen::filament_gltfio_FilamentAsset_popRenderables(
            self.native_mut(),
            core::ptr::null_mut(),
            0,
        );
        let mut result = vec![Entity::dangling(); len];
        bindgen::filament_gltfio_FilamentAsset_popRenderables(
            self.native_mut(),
            result.as_mut_ptr() as *mut _,
            len,
        );

        result
    }

    pub unsafe fn get_resource_uris(&self) -> Result<Vec<String>, Utf8Error> {
        let ptr = bindgen::filament_gltfio_FilamentAsset_getResourceUris(self.native());
        let len = bindgen::filament_gltfio_FilamentAsset_getResourceUriCount(self.native());

        let ptrs = core::slice::from_raw_parts(ptr, len);

        let mut result = Vec::with_capacity(ptrs.len());

        for ptr in ptrs {
            result.push(CStr::from_ptr(*ptr).to_str()?.to_owned())
        }

        return Ok(result);
    }

    pub unsafe fn get_bounding_box(&self) -> Aabb {
        let mut aabb = Aabb::default();
        bindgen::helper_filament_gltfio_filament_asset_get_bounding_box(
            self.native(),
            (&mut aabb) as *mut _ as *mut _,
        );
        aabb
    }

    pub unsafe fn get_name(&self, entity: Entity) -> Result<String, Utf8Error> {
        CStr::from_ptr(bindgen::filament_gltfio_FilamentAsset_getName(
            self.native(),
            entity.native_owned(),
        ))
        .to_str()
        .map(|s| s.to_owned())
    }

    // TODO: getFirstEntityByName
    // TODO: getEntitiesByName
    // TODO: getEntitiesByPrefix
    // TODO: getExtras
    // TODO: getAnimator
    // TODO: getMorphTargetNameAt
    // TODO: getWireframe
    // TODO: getEngine

    pub unsafe fn release_source_data(&mut self) {
        bindgen::filament_gltfio_FilamentAsset_releaseSourceData(self.native_mut())
    }

    // TODO: getSourceAsset
}
