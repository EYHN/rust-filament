pub mod ktx {
    use crate::{
        bindgen,
        filament::{Engine, Texture},
        image::KtxBundle,
    };

    pub unsafe fn create_texture(
        engine: &mut Engine,
        ktx: KtxBundle,
        srgb: bool,
    ) -> Option<Texture> {
        let result = Texture::try_from_native(bindgen::helper_image_ktx_createTexture(
            engine.native_mut(),
            ktx.native(),
            srgb,
            Some(create_texture_callback),
            Box::into_raw(Box::new(ktx)) as *mut _,
        ));
        result
    }

    unsafe extern "C" fn create_texture_callback(user: *mut std::ffi::c_void) {
        std::ptr::drop_in_place(user as *mut KtxBundle);
    }
}
