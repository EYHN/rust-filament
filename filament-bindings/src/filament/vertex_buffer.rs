use std::ptr;

use crate::{
    backend::{self, BufferDescriptor},
    bindgen,
};

use super::{Engine, VertexAttribute};

pub struct VertexBufferBuilder {
    native: bindgen::filament_VertexBuffer_Builder,
}

impl VertexBufferBuilder {
    #[inline]
    pub unsafe fn new() -> Self {
        Self {
            native: bindgen::filament_VertexBuffer_Builder::new(),
        }
    }

    #[inline]
    pub unsafe fn from(r: &VertexBufferBuilder) -> Self {
        Self {
            native: bindgen::filament_VertexBuffer_Builder::new1(&r.native),
        }
    }

    #[inline]
    pub unsafe fn buffer_count(&mut self, buffer_count: u8) -> &mut Self {
        self.native.bufferCount(buffer_count);
        self
    }

    #[inline]
    pub unsafe fn vertex_count(&mut self, vertex_count: u32) -> &mut Self {
        self.native.vertexCount(vertex_count);
        self
    }

    #[inline]
    pub unsafe fn enable_buffer_objects(&mut self, enabled: bool) -> &mut Self {
        self.native.enableBufferObjects(enabled);
        self
    }

    #[inline]
    pub unsafe fn attribute(
        &mut self,
        attribute: VertexAttribute,
        buffer_index: u8,
        attribute_type: backend::ElementType,
        byte_offset: u32,
        byte_stride: u8,
    ) -> &mut Self {
        self.native.attribute(
            attribute.into(),
            buffer_index,
            attribute_type.into(),
            byte_offset,
            byte_stride,
        );
        self
    }

    #[inline]
    pub unsafe fn normalized(&mut self, attribute: VertexAttribute, normalize: bool) -> &mut Self {
        self.native.normalized(attribute.into(), normalize);
        self
    }

    #[inline]
    pub unsafe fn build(&mut self, engine: &mut Engine) -> Option<VertexBuffer> {
        VertexBuffer::try_from_native(self.native.build(engine.native_mut()))
    }
}

impl Drop for VertexBufferBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.native.destruct() }
    }
}

#[derive(Clone)]
pub struct VertexBuffer {
    native: ptr::NonNull<bindgen::filament_VertexBuffer>,
}

impl VertexBuffer {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_VertexBuffer {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_VertexBuffer {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_VertexBuffer) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(VertexBuffer { native: ptr })
    }

    pub unsafe fn get_vertex_count(&self) -> usize {
        bindgen::filament_VertexBuffer_getVertexCount(self.native())
    }

    pub unsafe fn set_buffer_at<T>(
        &mut self,
        engine: &mut Engine,
        buffer_index: u8,
        buffer: BufferDescriptor<T>,
        byte_offset: u32,
    ) -> &mut Self {
        bindgen::filament_VertexBuffer_setBufferAt(
            self.native_mut(),
            engine.native_mut(),
            buffer_index,
            &mut buffer.into_native(),
            byte_offset,
        );
        self
    }

    // TODO: set_buffer_object_at
}
