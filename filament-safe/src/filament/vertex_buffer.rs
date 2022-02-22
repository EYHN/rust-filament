use std::ptr;

use filament_bindings::{
    filament_Engine_destroy3, filament_VertexBuffer, filament_VertexBuffer_Builder,
    filament_VertexBuffer_getVertexCount, filament_VertexBuffer_setBufferAt,
};

use crate::{
    backend::{self, BufferDescriptor},
    prelude::{EngineData, EngineDrop, EngineResult, NativeHandle, RcHandle},
};

use super::{Engine, VertexAttribute, WeakEngine};

pub struct VertexBufferBuilder {
    native: filament_VertexBuffer_Builder,
}

impl VertexBufferBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            native: unsafe { filament_VertexBuffer_Builder::new() },
        }
    }

    #[inline]
    pub fn from(r: &VertexBufferBuilder) -> Self {
        Self {
            native: unsafe { filament_VertexBuffer_Builder::new1(&r.native) },
        }
    }

    #[inline]
    pub fn buffer_count(&mut self, buffer_count: u8) -> &mut Self {
        unsafe { self.native.bufferCount(buffer_count) };
        self
    }

    #[inline]
    pub fn vertex_count(&mut self, vertex_count: u32) -> &mut Self {
        unsafe { self.native.vertexCount(vertex_count) };
        self
    }

    #[inline]
    pub fn enable_buffer_objects(&mut self, enabled: bool) -> &mut Self {
        unsafe { self.native.enableBufferObjects(enabled) };
        self
    }

    #[inline]
    pub fn attribute(
        &mut self,
        attribute: VertexAttribute,
        buffer_index: u8,
        attribute_type: backend::ElementType,
        byte_offset: u32,
        byte_stride: u8,
    ) -> &mut Self {
        unsafe {
            self.native.attribute(
                attribute.into(),
                buffer_index,
                attribute_type.into(),
                byte_offset,
                byte_stride,
            )
        };
        self
    }

    #[inline]
    pub fn normalized(&mut self, attribute: VertexAttribute, normalize: bool) -> &mut Self {
        unsafe { self.native.normalized(attribute.into(), normalize) };
        self
    }

    #[inline]
    pub fn build(&mut self, engine: &mut Engine) -> Option<VertexBuffer> {
        VertexBuffer::try_from_native(engine.downgrade(), unsafe {
            self.native.build(engine.native_mut())
        })
    }
}

impl Drop for VertexBufferBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.native.destruct() }
    }
}

#[derive(Clone)]
pub struct VertexBufferData {
    native: ptr::NonNull<filament_VertexBuffer>,
}

pub type VertexBuffer = RcHandle<EngineData<VertexBufferData>>;

impl NativeHandle<filament_VertexBuffer> for VertexBuffer {
    #[inline]
    fn native(&self) -> *const filament_VertexBuffer {
        self.data().native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_VertexBuffer {
        self.data_mut().native.as_ptr()
    }
}

impl VertexBuffer {
    #[inline]
    pub(crate) fn try_from_native(
        engine: WeakEngine,
        native: *mut filament_VertexBuffer,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self::new(EngineData::new(
            VertexBufferData { native: ptr },
            engine,
        )))
    }
}

impl VertexBuffer {
    pub fn get_vertex_count(&self) -> usize {
        unsafe { filament_VertexBuffer_getVertexCount(self.native()) as usize }
    }

    pub fn set_buffer_at<T>(
        &mut self,
        buffer_index: u8,
        buffer: BufferDescriptor<T>,
        byte_offset: u32,
    ) -> EngineResult<&mut Self> {
        unsafe {
            filament_VertexBuffer_setBufferAt(
                self.native_mut(),
                self.engine().upgrade_engine()?.native_mut(),
                buffer_index,
                &mut buffer.into_native(),
                byte_offset,
            )
        };
        Ok(self)
    }

    // TODO: set_buffer_object_at
}

impl EngineDrop for VertexBufferData {
    fn drop(&mut self, engine: &mut Engine) {
        unsafe { filament_Engine_destroy3(engine.native_mut(), self.native.as_ptr()) };
    }
}
