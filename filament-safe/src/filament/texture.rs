use std::{ffi, ptr, rc::Rc};

use filament_bindings::{
    filament_Engine_destroy17, filament_Texture, filament_Texture_BASE_LEVEL,
    filament_Texture_Builder, filament_Texture_generateMipmaps, filament_Texture_getDepth,
    filament_Texture_getFormat, filament_Texture_getHeight, filament_Texture_getLevels,
    filament_Texture_getTarget, filament_Texture_getWidth, filament_Texture_setExternalImage,
    filament_Texture_setExternalImage1, filament_Texture_setImage, filament_Texture_setImage1,
    filament_Texture_setImage2, size_t,
};

use crate::{
    backend::{PixelBufferDescriptor, SamplerType, TextureFormat, TextureSwizzle, TextureUsage},
    prelude::NativeHandle,
};

use super::Engine;

pub struct TextureBuilder {
    native: filament_Texture_Builder,
}

impl TextureBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            native: unsafe { filament_Texture_Builder::new() },
        }
    }

    #[inline]
    pub fn from(r: &TextureBuilder) -> Self {
        Self {
            native: unsafe { filament_Texture_Builder::new1(&r.native) },
        }
    }

    #[inline]
    pub fn width(&mut self, width: u32) -> &mut Self {
        unsafe { self.native.width(width) };
        self
    }

    #[inline]
    pub fn height(&mut self, height: u32) -> &mut Self {
        unsafe { self.native.height(height) };
        self
    }

    #[inline]
    pub fn depth(&mut self, depth: u32) -> &mut Self {
        unsafe { self.native.depth(depth) };
        self
    }

    #[inline]
    pub fn levels(&mut self, levels: u8) -> &mut Self {
        unsafe { self.native.levels(levels) };
        self
    }

    #[inline]
    pub fn sampler(&mut self, target: SamplerType) -> &mut Self {
        unsafe { self.native.sampler(target.into()) };
        self
    }

    #[inline]
    pub fn format(&mut self, format: TextureFormat) -> &mut Self {
        unsafe { self.native.format(format.into()) };
        self
    }

    #[inline]
    pub fn usage(&mut self, usage: TextureUsage) -> &mut Self {
        unsafe { self.native.usage(usage.bits()) };
        self
    }

    #[inline]
    pub fn swizzle(
        &mut self,
        r: TextureSwizzle,
        g: TextureSwizzle,
        b: TextureSwizzle,
        a: TextureSwizzle,
    ) -> &mut Self {
        unsafe { self.native.swizzle(r.into(), g.into(), b.into(), a.into()) };
        self
    }

    #[inline]
    pub fn build(&mut self, engine: &mut Engine) -> Option<Texture> {
        Texture::try_from_native(engine.clone(), unsafe {
            self.native.build(engine.native_mut())
        })
    }
}

impl Drop for TextureBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.native.destruct() }
    }
}

#[derive(Clone)]
pub struct Texture {
    native: Rc<ptr::NonNull<filament_Texture>>,
    engine: Engine,
}

impl NativeHandle<filament_Texture> for Texture {
    #[inline]
    fn native(&self) -> *const filament_Texture {
        self.native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_Texture {
        self.native.as_ptr()
    }
}

impl Texture {
    #[inline]
    pub(crate) fn try_from_native(engine: Engine, native: *mut filament_Texture) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self {
            native: Rc::new(ptr),
            engine,
        })
    }
}

impl Texture {
    #[inline]
    pub fn get_width(&self) -> usize {
        unsafe { filament_Texture_getWidth(self.native(), filament_Texture_BASE_LEVEL) as usize }
    }

    #[inline]
    pub fn get_width_level(&self, level: usize) -> usize {
        unsafe { filament_Texture_getWidth(self.native(), level as size_t) as usize }
    }

    #[inline]
    pub fn get_height(&self) -> usize {
        unsafe { filament_Texture_getHeight(self.native(), filament_Texture_BASE_LEVEL) as usize }
    }

    #[inline]
    pub fn get_height_level(&self, level: usize) -> usize {
        unsafe { filament_Texture_getHeight(self.native(), level as size_t) as usize }
    }

    #[inline]
    pub fn get_depth(&self) -> usize {
        unsafe { filament_Texture_getDepth(self.native(), filament_Texture_BASE_LEVEL) as usize }
    }

    #[inline]
    pub fn get_depth_level(&self, level: usize) -> usize {
        unsafe { filament_Texture_getDepth(self.native(), level as size_t) as usize }
    }

    #[inline]
    pub fn get_levels(&self) -> usize {
        unsafe { filament_Texture_getLevels(self.native()) as usize }
    }

    #[inline]
    pub fn get_target(&self) -> SamplerType {
        unsafe { SamplerType::from(filament_Texture_getTarget(self.native())) }
    }

    #[inline]
    pub fn get_format(&self) -> TextureFormat {
        unsafe { TextureFormat::from(filament_Texture_getFormat(self.native())) }
    }

    #[inline]
    pub fn set_image<T>(&mut self, level: usize, buffer: PixelBufferDescriptor<T>) -> &mut Self {
        unsafe {
            filament_Texture_setImage(
                self.native_mut(),
                self.engine.native_mut(),
                level as size_t,
                &mut buffer.into_native(),
            )
        };
        self
    }

    #[inline]
    pub fn set_image_offset_size<T>(
        &mut self,
        level: usize,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: PixelBufferDescriptor<T>,
    ) -> &mut Self {
        unsafe {
            filament_Texture_setImage1(
                self.native_mut(),
                self.engine.native_mut(),
                level as size_t,
                xoffset,
                yoffset,
                width,
                height,
                &mut buffer.into_native(),
            )
        };
        self
    }

    #[inline]
    pub fn set_image_offset_size_depth<T>(
        &mut self,
        level: usize,
        xoffset: u32,
        yoffset: u32,
        zoffset: u32,
        width: u32,
        height: u32,
        depth: u32,
        buffer: PixelBufferDescriptor<T>,
    ) -> &mut Self {
        unsafe {
            filament_Texture_setImage2(
                self.native_mut(),
                self.engine.native_mut(),
                level as size_t,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                &mut buffer.into_native(),
            )
        };
        self
    }

    // TODO: set image face offset

    #[inline]
    pub fn set_external_image(&mut self, image: *mut ffi::c_void) -> &mut Self {
        unsafe {
            filament_Texture_setExternalImage(self.native_mut(), self.engine.native_mut(), image)
        };
        self
    }

    #[inline]
    pub fn set_external_image_plane(&mut self, image: *mut ffi::c_void, plane: usize) -> &mut Self {
        unsafe {
            filament_Texture_setExternalImage1(
                self.native_mut(),
                self.engine.native_mut(),
                image,
                plane as size_t,
            )
        };
        self
    }

    // TODO: setExternalStream

    #[inline]
    pub fn generate_mipmaps(&mut self) -> &mut Self {
        unsafe { filament_Texture_generateMipmaps(self.native_mut(), self.engine.native_mut()) };
        self
    }

    // TODO: generatePrefilterMipmap
}

impl Drop for Texture {
    #[inline]
    fn drop(&mut self) {
        if let Some(_) = Rc::get_mut(&mut self.native) {
            unsafe { filament_Engine_destroy17(self.engine.native_mut(), self.native()) };
        }
    }
}
