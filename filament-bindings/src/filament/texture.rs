use std::{ffi, ptr};

use crate::{
    backend::{PixelBufferDescriptor, SamplerType, TextureFormat, TextureSwizzle, TextureUsage},
    bindgen,
};

use super::Engine;

pub struct TextureBuilder {
    native: bindgen::filament_Texture_Builder,
}

impl TextureBuilder {
    #[inline]
    pub unsafe fn new() -> Self {
        Self {
            native: bindgen::filament_Texture_Builder::new(),
        }
    }

    #[inline]
    pub unsafe fn from(r: &TextureBuilder) -> Self {
        Self {
            native: bindgen::filament_Texture_Builder::new1(&r.native),
        }
    }

    #[inline]
    pub unsafe fn width(&mut self, width: u32) -> &mut Self {
        self.native.width(width);
        self
    }

    #[inline]
    pub unsafe fn height(&mut self, height: u32) -> &mut Self {
        self.native.height(height);
        self
    }

    #[inline]
    pub unsafe fn depth(&mut self, depth: u32) -> &mut Self {
        self.native.depth(depth);
        self
    }

    #[inline]
    pub unsafe fn levels(&mut self, levels: u8) -> &mut Self {
        self.native.levels(levels);
        self
    }

    #[inline]
    pub unsafe fn sampler(&mut self, target: SamplerType) -> &mut Self {
        self.native.sampler(target.into());
        self
    }

    #[inline]
    pub unsafe fn format(&mut self, format: TextureFormat) -> &mut Self {
        self.native.format(format.into());
        self
    }

    #[inline]
    pub unsafe fn usage(&mut self, usage: TextureUsage) -> &mut Self {
        self.native.usage(usage.bits());
        self
    }

    #[inline]
    pub unsafe fn swizzle(
        &mut self,
        r: TextureSwizzle,
        g: TextureSwizzle,
        b: TextureSwizzle,
        a: TextureSwizzle,
    ) -> &mut Self {
        self.native.swizzle(r.into(), g.into(), b.into(), a.into());
        self
    }

    #[inline]
    pub unsafe fn build(&mut self, engine: &mut Engine) -> Option<Texture> {
        Texture::try_from_native(self.native.build(engine.native_mut()))
    }
}

impl Drop for TextureBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.native.destruct() }
    }
}

pub struct Texture {
    native: ptr::NonNull<bindgen::filament_Texture>,
}

impl Texture {
    #[inline]
    pub unsafe fn native(&self) -> *const bindgen::filament_Texture {
        self.native.as_ptr()
    }

    #[inline]
    pub unsafe fn native_mut(&mut self) -> *mut bindgen::filament_Texture {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_Texture) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Texture { native: ptr })
    }

    #[inline]
    pub unsafe fn get_width(&self) -> usize {
        bindgen::filament_Texture_getWidth(self.native(), bindgen::filament_Texture_BASE_LEVEL)
    }

    #[inline]
    pub unsafe fn get_width_level(&self, level: usize) -> usize {
        bindgen::filament_Texture_getWidth(self.native(), level)
    }

    #[inline]
    pub unsafe fn get_height(&self) -> usize {
        bindgen::filament_Texture_getHeight(self.native(), bindgen::filament_Texture_BASE_LEVEL)
    }

    #[inline]
    pub unsafe fn get_height_level(&self, level: usize) -> usize {
        bindgen::filament_Texture_getHeight(self.native(), level)
    }

    #[inline]
    pub unsafe fn get_depth(&self) -> usize {
        bindgen::filament_Texture_getDepth(self.native(), bindgen::filament_Texture_BASE_LEVEL)
    }

    #[inline]
    pub unsafe fn get_depth_level(&self, level: usize) -> usize {
        bindgen::filament_Texture_getDepth(self.native(), level)
    }

    #[inline]
    pub unsafe fn get_levels(&self) -> usize {
        bindgen::filament_Texture_getLevels(self.native())
    }

    #[inline]
    pub unsafe fn get_target(&self) -> SamplerType {
        SamplerType::from(bindgen::filament_Texture_getTarget(self.native()))
    }

    #[inline]
    pub unsafe fn get_format(&self) -> TextureFormat {
        TextureFormat::from(bindgen::filament_Texture_getFormat(self.native()))
    }

    #[inline]
    pub unsafe fn set_image<T>(
        &mut self,
        engine: &mut Engine,
        level: usize,
        buffer: PixelBufferDescriptor<T>,
    ) -> &mut Self {
        bindgen::filament_Texture_setImage(
            self.native_mut(),
            engine.native_mut(),
            level,
            &mut buffer.into_native(),
        );
        self
    }

    #[inline]
    pub unsafe fn set_image_offset_size<T>(
        &mut self,
        engine: &mut Engine,
        level: usize,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: PixelBufferDescriptor<T>,
    ) -> &mut Self {
        bindgen::filament_Texture_setImage1(
            self.native_mut(),
            engine.native_mut(),
            level,
            xoffset,
            yoffset,
            width,
            height,
            &mut buffer.into_native(),
        );
        self
    }

    #[inline]
    pub unsafe fn set_image_offset_size_depth<T>(
        &mut self,
        engine: &mut Engine,
        level: usize,
        xoffset: u32,
        yoffset: u32,
        zoffset: u32,
        width: u32,
        height: u32,
        depth: u32,
        buffer: PixelBufferDescriptor<T>,
    ) -> &mut Self {
        bindgen::filament_Texture_setImage2(
            self.native_mut(),
            engine.native_mut(),
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            &mut buffer.into_native(),
        );
        self
    }

    // TODO: set image face offset

    #[inline]
    pub unsafe fn set_external_image(
        &mut self,
        engine: &mut Engine,
        image: *mut ffi::c_void,
    ) -> &mut Self {
        bindgen::filament_Texture_setExternalImage(self.native_mut(), engine.native_mut(), image);
        self
    }

    #[inline]
    pub unsafe fn set_external_image_plane(
        &mut self,
        engine: &mut Engine,
        image: *mut ffi::c_void,
        plane: usize,
    ) -> &mut Self {
        bindgen::filament_Texture_setExternalImage1(
            self.native_mut(),
            engine.native_mut(),
            image,
            plane,
        );
        self
    }

    // TODO: setExternalStream

    #[inline]
    pub unsafe fn generate_mipmaps(&mut self, engine: &mut Engine) -> &mut Self {
        bindgen::filament_Texture_generateMipmaps(self.native_mut(), engine.native_mut());
        self
    }

    // TODO: generatePrefilterMipmap
}
