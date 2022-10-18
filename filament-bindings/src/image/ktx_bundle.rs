#![allow(non_upper_case_globals)]

use std::ptr;

use crate::{bindgen, math::Float3};

#[repr(C)]
#[derive(Debug)]
pub struct Ktx1Bundle {
    pub(crate) native: ptr::NonNull<bindgen::image_Ktx1Bundle>,
}

impl Ktx1Bundle {
    #[inline]
    pub(crate) unsafe fn native(&self) -> *const bindgen::image_Ktx1Bundle {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn native_mut(&mut self) -> *mut bindgen::image_Ktx1Bundle {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn try_from_native(native: *mut bindgen::image_Ktx1Bundle) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Ktx1Bundle { native: ptr })
    }

    #[inline]
    pub unsafe fn new(num_mip_levels: u32, array_length: u32, is_cubemap: bool) -> Option<Self> {
        Self::try_from_native(bindgen::helper_image_ktx1_bundle_create(
            num_mip_levels,
            array_length,
            is_cubemap,
        ))
    }

    #[inline]
    pub unsafe fn from(data: &[u8]) -> Option<Self> {
        Self::try_from_native(bindgen::helper_image_ktx1_bundle_from(
            data.as_ptr(),
            data.len() as u32,
        ))
    }

    #[inline]
    pub unsafe fn serialize(&self) -> Option<Vec<u8>> {
        let serialized_length = bindgen::image_Ktx1Bundle_getSerializedLength(self.native());
        let mut destination = vec![0u8; serialized_length as usize];
        if bindgen::image_Ktx1Bundle_serialize(
            self.native(),
            destination.as_mut_ptr(),
            serialized_length,
        ) {
            Some(destination)
        } else {
            None
        }
    }

    // TODO:
    // #[inline]
    // pub unsafe fn getMetadata(
    //     &self,
    //     key: *const ::std::os::raw::c_char,
    //     valueSize: *mut usize,
    // ) -> *const ::std::os::raw::c_char {
    //     bindgen::image_Ktx1Bundle_getMetadata(self.native(), key, valueSize)
    // }

    // #[inline]
    // pub unsafe fn set_metadata(
    //     &mut self,
    //     key: *const ::std::os::raw::c_char,
    //     value: *const ::std::os::raw::c_char,
    // ) {
    //     bindgen::image_Ktx1Bundle_setMetadata(self.native_mut(), key, value)
    // }

    #[inline]
    pub unsafe fn get_spherical_harmonics(&mut self) -> Option<Float3> {
        let mut result = Float3::default();
        if bindgen::image_Ktx1Bundle_getSphericalHarmonics(
            self.native_mut(),
            result.native_ptr_mut(),
        ) {
            Some(result)
        } else {
            None
        }
    }

    // TODO:
    // #[inline]
    // pub unsafe fn getBlob(
    //     &self,
    //     index: bindgen::image_KtxBlobIndex,
    //     data: *mut *mut u8,
    //     size: *mut u32,
    // ) -> bool {
    //     bindgen::image_Ktx1Bundle_getBlob(self.native(), index, data, size)
    // }

    // #[inline]
    // pub unsafe fn setBlob(
    //     &mut self,
    //     index: bindgen::image_KtxBlobIndex,
    //     data: *const u8,
    //     size: u32,
    // ) -> bool {
    //     bindgen::image_Ktx1Bundle_setBlob(self.native_mut(), index, data, size)
    // }

    #[inline]
    pub unsafe fn allocate_blob(&mut self, index: bindgen::image_KtxBlobIndex, size: u32) -> bool {
        bindgen::image_Ktx1Bundle_allocateBlob(self.native_mut(), index, size)
    }

    pub const R8: u32 = bindgen::image_Ktx1Bundle_R8;
    pub const R8_SNORM: u32 = bindgen::image_Ktx1Bundle_R8_SNORM;
    pub const R8UI: u32 = bindgen::image_Ktx1Bundle_R8UI;
    pub const R8I: u32 = bindgen::image_Ktx1Bundle_R8I;
    pub const STENCIL_INDEX8: u32 = bindgen::image_Ktx1Bundle_STENCIL_INDEX8;
    pub const R16F: u32 = bindgen::image_Ktx1Bundle_R16F;
    pub const R16UI: u32 = bindgen::image_Ktx1Bundle_R16UI;
    pub const R16I: u32 = bindgen::image_Ktx1Bundle_R16I;
    pub const RG8: u32 = bindgen::image_Ktx1Bundle_RG8;
    pub const RG8_SNORM: u32 = bindgen::image_Ktx1Bundle_RG8_SNORM;
    pub const RG8UI: u32 = bindgen::image_Ktx1Bundle_RG8UI;
    pub const RG8I: u32 = bindgen::image_Ktx1Bundle_RG8I;
    pub const RGB565: u32 = bindgen::image_Ktx1Bundle_RGB565;
    pub const RGB5_A1: u32 = bindgen::image_Ktx1Bundle_RGB5_A1;
    pub const RGBA4: u32 = bindgen::image_Ktx1Bundle_RGBA4;
    pub const DEPTH_COMPONENT16: u32 = bindgen::image_Ktx1Bundle_DEPTH_COMPONENT16;
    pub const RGB8: u32 = bindgen::image_Ktx1Bundle_RGB8;
    pub const SRGB8: u32 = bindgen::image_Ktx1Bundle_SRGB8;
    pub const RGB8_SNORM: u32 = bindgen::image_Ktx1Bundle_RGB8_SNORM;
    pub const RGB8UI: u32 = bindgen::image_Ktx1Bundle_RGB8UI;
    pub const RGB8I: u32 = bindgen::image_Ktx1Bundle_RGB8I;
    pub const DEPTH_COMPONENT24: u32 = bindgen::image_Ktx1Bundle_DEPTH_COMPONENT24;
    pub const R32F: u32 = bindgen::image_Ktx1Bundle_R32F;
    pub const R32UI: u32 = bindgen::image_Ktx1Bundle_R32UI;
    pub const R32I: u32 = bindgen::image_Ktx1Bundle_R32I;
    pub const RG16F: u32 = bindgen::image_Ktx1Bundle_RG16F;
    pub const RG16UI: u32 = bindgen::image_Ktx1Bundle_RG16UI;
    pub const RG16I: u32 = bindgen::image_Ktx1Bundle_RG16I;
    pub const R11F_G11F_B10F: u32 = bindgen::image_Ktx1Bundle_R11F_G11F_B10F;
    pub const RGB9_E5: u32 = bindgen::image_Ktx1Bundle_RGB9_E5;
    pub const RGBA8: u32 = bindgen::image_Ktx1Bundle_RGBA8;
    pub const SRGB8_ALPHA8: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8;
    pub const RGBA8_SNORM: u32 = bindgen::image_Ktx1Bundle_RGBA8_SNORM;
    pub const RGB10_A2: u32 = bindgen::image_Ktx1Bundle_RGB10_A2;
    pub const RGBA8UI: u32 = bindgen::image_Ktx1Bundle_RGBA8UI;
    pub const RGBA8I: u32 = bindgen::image_Ktx1Bundle_RGBA8I;
    pub const DEPTH_COMPONENT32F: u32 = bindgen::image_Ktx1Bundle_DEPTH_COMPONENT32F;
    pub const DEPTH24_STENCIL8: u32 = bindgen::image_Ktx1Bundle_DEPTH24_STENCIL8;
    pub const DEPTH32F_STENCIL8: u32 = bindgen::image_Ktx1Bundle_DEPTH32F_STENCIL8;
    pub const RGB16F: u32 = bindgen::image_Ktx1Bundle_RGB16F;
    pub const RGB16UI: u32 = bindgen::image_Ktx1Bundle_RGB16UI;
    pub const RGB16I: u32 = bindgen::image_Ktx1Bundle_RGB16I;
    pub const RG32F: u32 = bindgen::image_Ktx1Bundle_RG32F;
    pub const RG32UI: u32 = bindgen::image_Ktx1Bundle_RG32UI;
    pub const RG32I: u32 = bindgen::image_Ktx1Bundle_RG32I;
    pub const RGBA16F: u32 = bindgen::image_Ktx1Bundle_RGBA16F;
    pub const RGBA16UI: u32 = bindgen::image_Ktx1Bundle_RGBA16UI;
    pub const RGBA16I: u32 = bindgen::image_Ktx1Bundle_RGBA16I;
    pub const RGB32F: u32 = bindgen::image_Ktx1Bundle_RGB32F;
    pub const RGB32UI: u32 = bindgen::image_Ktx1Bundle_RGB32UI;
    pub const RGB32I: u32 = bindgen::image_Ktx1Bundle_RGB32I;
    pub const RGBA32F: u32 = bindgen::image_Ktx1Bundle_RGBA32F;
    pub const RGBA32UI: u32 = bindgen::image_Ktx1Bundle_RGBA32UI;
    pub const RGBA32I: u32 = bindgen::image_Ktx1Bundle_RGBA32I;

    pub const RED: u32 = bindgen::image_Ktx1Bundle_RED;
    pub const RG: u32 = bindgen::image_Ktx1Bundle_RG;
    pub const RGB: u32 = bindgen::image_Ktx1Bundle_RGB;
    pub const RGBA: u32 = bindgen::image_Ktx1Bundle_RGBA;
    pub const BGR: u32 = bindgen::image_Ktx1Bundle_BGR;
    pub const BGRA: u32 = bindgen::image_Ktx1Bundle_BGRA;
    pub const LUMINANCE: u32 = bindgen::image_Ktx1Bundle_LUMINANCE;
    pub const LUMINANCE_ALPHA: u32 = bindgen::image_Ktx1Bundle_LUMINANCE_ALPHA;

    pub const UNSIGNED_BYTE: u32 = bindgen::image_Ktx1Bundle_UNSIGNED_BYTE;
    pub const UNSIGNED_SHORT: u32 = bindgen::image_Ktx1Bundle_UNSIGNED_SHORT;
    pub const HALF_FLOAT: u32 = bindgen::image_Ktx1Bundle_HALF_FLOAT;
    pub const FLOAT: u32 = bindgen::image_Ktx1Bundle_FLOAT;

    pub const ENDIAN_DEFAULT: u32 = bindgen::image_Ktx1Bundle_ENDIAN_DEFAULT;

    pub const RGB_S3TC_DXT1: u32 = bindgen::image_Ktx1Bundle_RGB_S3TC_DXT1;
    pub const RGBA_S3TC_DXT1: u32 = bindgen::image_Ktx1Bundle_RGBA_S3TC_DXT1;
    pub const RGBA_S3TC_DXT3: u32 = bindgen::image_Ktx1Bundle_RGBA_S3TC_DXT3;
    pub const RGBA_S3TC_DXT5: u32 = bindgen::image_Ktx1Bundle_RGBA_S3TC_DXT5;

    pub const RGBA_ASTC_4x4: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_4x4;
    pub const RGBA_ASTC_5x4: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_5x4;
    pub const RGBA_ASTC_5x5: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_5x5;
    pub const RGBA_ASTC_6x5: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_6x5;
    pub const RGBA_ASTC_6x6: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_6x6;
    pub const RGBA_ASTC_8x5: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_8x5;
    pub const RGBA_ASTC_8x6: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_8x6;
    pub const RGBA_ASTC_8x8: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_8x8;
    pub const RGBA_ASTC_10x5: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_10x5;
    pub const RGBA_ASTC_10x6: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_10x6;
    pub const RGBA_ASTC_10x8: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_10x8;
    pub const RGBA_ASTC_10x10: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_10x10;
    pub const RGBA_ASTC_12x10: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_12x10;
    pub const RGBA_ASTC_12x12: u32 = bindgen::image_Ktx1Bundle_RGBA_ASTC_12x12;
    pub const SRGB8_ALPHA8_ASTC_4x4: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_4x4;
    pub const SRGB8_ALPHA8_ASTC_5x4: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_5x4;
    pub const SRGB8_ALPHA8_ASTC_5x5: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_5x5;
    pub const SRGB8_ALPHA8_ASTC_6x5: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_6x5;
    pub const SRGB8_ALPHA8_ASTC_6x6: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_6x6;
    pub const SRGB8_ALPHA8_ASTC_8x5: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_8x5;
    pub const SRGB8_ALPHA8_ASTC_8x6: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_8x6;
    pub const SRGB8_ALPHA8_ASTC_8x8: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_8x8;
    pub const SRGB8_ALPHA8_ASTC_10x5: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_10x5;
    pub const SRGB8_ALPHA8_ASTC_10x6: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_10x6;
    pub const SRGB8_ALPHA8_ASTC_10x8: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_10x8;
    pub const SRGB8_ALPHA8_ASTC_10x10: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_10x10;
    pub const SRGB8_ALPHA8_ASTC_12x10: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_12x10;
    pub const SRGB8_ALPHA8_ASTC_12x12: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ASTC_12x12;

    pub const R11_EAC: u32 = bindgen::image_Ktx1Bundle_R11_EAC;
    pub const SIGNED_R11_EAC: u32 = bindgen::image_Ktx1Bundle_SIGNED_R11_EAC;
    pub const RG11_EAC: u32 = bindgen::image_Ktx1Bundle_RG11_EAC;
    pub const SIGNED_RG11_EAC: u32 = bindgen::image_Ktx1Bundle_SIGNED_RG11_EAC;
    pub const RGB8_ETC2: u32 = bindgen::image_Ktx1Bundle_RGB8_ETC2;
    pub const SRGB8_ETC2: u32 = bindgen::image_Ktx1Bundle_SRGB8_ETC2;
    pub const RGB8_ALPHA1_ETC2: u32 = bindgen::image_Ktx1Bundle_RGB8_ALPHA1_ETC2;
    pub const SRGB8_ALPHA1_ETC: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA1_ETC;
    pub const RGBA8_ETC2_EAC: u32 = bindgen::image_Ktx1Bundle_RGBA8_ETC2_EAC;
    pub const SRGB8_ALPHA8_ETC2_EAC: u32 = bindgen::image_Ktx1Bundle_SRGB8_ALPHA8_ETC2_EAC;
}

impl Drop for Ktx1Bundle {
    #[inline]
    fn drop(&mut self) {
        unsafe { bindgen::helper_image_ktx1_bundle_delete(self.native_mut()) }
    }
}
