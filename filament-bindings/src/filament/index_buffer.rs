use std::ptr;

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{backend::BufferDescriptor, bindgen};

use super::Engine;

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum IndexType {
    USHORT = bindgen::filament_IndexBuffer_IndexType_USHORT,
    UINT = bindgen::filament_IndexBuffer_IndexType_UINT,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[repr(transparent)]
pub struct IndexBufferBuilder {
    native: bindgen::filament_IndexBuffer_Builder,
}

impl IndexBufferBuilder {
    #[inline]
    pub unsafe fn new() -> Self {
        Self {
            native: bindgen::filament_IndexBuffer_Builder::new(),
        }
    }

    #[inline]
    pub unsafe fn from(r: &IndexBufferBuilder) -> Self {
        Self {
            native: bindgen::filament_IndexBuffer_Builder::new1(&r.native),
        }
    }

    #[inline]
    pub unsafe fn index_count(&mut self, index_count: u32) -> &mut Self {
        self.native.indexCount(index_count);
        self
    }

    #[inline]
    pub unsafe fn buffer_type(&mut self, index_type: IndexType) -> &mut Self {
        self.native.bufferType(index_type.into());
        self
    }

    #[inline]
    pub unsafe fn build(&mut self, engine: &mut Engine) -> Option<IndexBuffer> {
        IndexBuffer::try_from_native(self.native.build(engine.native_mut()))
    }
}

impl Drop for IndexBufferBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.native.destruct() }
    }
}

pub struct IndexBuffer {
    native: ptr::NonNull<bindgen::filament_IndexBuffer>,
}

impl IndexBuffer {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_IndexBuffer {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_IndexBuffer {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_IndexBuffer) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(IndexBuffer { native: ptr })
    }

    #[inline]
    pub unsafe fn get_index_count(&self) -> usize {
        bindgen::filament_IndexBuffer_getIndexCount(self.native())
    }

    #[inline]
    pub unsafe fn set_buffer<T>(
        &mut self,
        engine: &mut Engine,
        buffer: BufferDescriptor<T>,
        byte_offset: u32,
    ) -> &mut Self {
        bindgen::filament_IndexBuffer_setBuffer(
            self.native_mut(),
            engine.native_mut(),
            &mut buffer.into_native(),
            byte_offset,
        );
        self
    }
}
