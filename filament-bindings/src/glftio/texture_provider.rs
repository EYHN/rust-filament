use std::ptr;

use crate::bindgen;

pub struct TextureProvider {
    native: ptr::NonNull<bindgen::filament_gltfio_TextureProvider>,
}

impl TextureProvider {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_gltfio_TextureProvider {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_gltfio_TextureProvider {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_gltfio_TextureProvider) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(TextureProvider { native: ptr })
    }
}

impl Drop for TextureProvider {
    fn drop(&mut self) {
        unsafe { bindgen::helper_filament_gltfio_texture_provider_delete(self.native_mut()) }
    }
}

#[inline]
pub unsafe fn create_stb_provider(engine: &mut crate::filament::Engine) -> Option<TextureProvider> {
    let ptr = ptr::NonNull::new(bindgen::helper_filament_gltfio_create_stb_provider(
        engine.native_mut(),
    ))?;
    Some(TextureProvider { native: ptr })
}

#[inline]
pub unsafe fn create_ktx2_provider(
    engine: &mut crate::filament::Engine,
) -> Option<TextureProvider> {
    let ptr = ptr::NonNull::new(bindgen::helper_filament_gltfio_create_ktx2_provider(
        engine.native_mut(),
    ))?;
    Some(TextureProvider { native: ptr })
}
