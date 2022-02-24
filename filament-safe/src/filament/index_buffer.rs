use std::ptr;

use filament_bindings::{
    filament_Engine_destroy5, filament_IndexBuffer, filament_IndexBuffer_Builder,
    filament_IndexBuffer_IndexType_UINT, filament_IndexBuffer_IndexType_USHORT,
    filament_IndexBuffer_getIndexCount, filament_IndexBuffer_setBuffer,
};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    backend::BufferDescriptor,
    prelude::{EngineData, EngineDrop, EngineResult, NativeHandle, RcHandle},
};

use super::{Engine, WeakEngine};

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum IndexType {
    USHORT = filament_IndexBuffer_IndexType_USHORT,
    UINT = filament_IndexBuffer_IndexType_UINT,
    #[num_enum(default)]
    UNKNOWN = 255,
}

pub struct IndexBufferBuilder {
    native: filament_IndexBuffer_Builder,
}

impl IndexBufferBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            native: unsafe { filament_IndexBuffer_Builder::new() },
        }
    }

    #[inline]
    pub fn from(r: &IndexBufferBuilder) -> Self {
        Self {
            native: unsafe { filament_IndexBuffer_Builder::new1(&r.native) },
        }
    }

    #[inline]
    pub fn index_count(&mut self, index_count: u32) -> &mut Self {
        unsafe { self.native.indexCount(index_count) };
        self
    }

    #[inline]
    pub fn buffer_type(&mut self, index_type: IndexType) -> &mut Self {
        unsafe { self.native.bufferType(index_type.into()) };
        self
    }

    #[inline]
    pub fn build(&mut self, engine: &mut Engine) -> Option<IndexBuffer> {
        IndexBuffer::try_from_native(engine.downgrade(), unsafe {
            self.native.build(engine.native_mut())
        })
    }
}

impl Drop for IndexBufferBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.native.destruct() }
    }
}

pub struct IndexBufferInner {
    native: ptr::NonNull<filament_IndexBuffer>,
}

pub type IndexBuffer = RcHandle<EngineData<IndexBufferInner>>;

impl NativeHandle<filament_IndexBuffer> for IndexBuffer {
    #[inline]
    fn native(&self) -> *const filament_IndexBuffer {
        self.data().native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_IndexBuffer {
        self.data_mut().native.as_ptr()
    }
}

impl IndexBuffer {
    #[inline]
    pub(crate) fn try_from_native(
        engine: WeakEngine,
        native: *mut filament_IndexBuffer,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(RcHandle::new(EngineData::new(
            IndexBufferInner { native: ptr },
            engine,
        )))
    }
}

impl IndexBuffer {
    #[inline]
    pub fn get_index_count(&self) -> usize {
        unsafe { filament_IndexBuffer_getIndexCount(self.native()) as usize }
    }

    #[inline]
    pub fn set_buffer<T>(
        &mut self,
        buffer: BufferDescriptor<T>,
        byte_offset: u32,
    ) -> EngineResult<&mut Self> {
        unsafe {
            filament_IndexBuffer_setBuffer(
                self.native_mut(),
                self.engine().upgrade_engine()?.native_mut(),
                &mut buffer.into_native(),
                byte_offset,
            )
        };
        Ok(self)
    }
}

impl EngineDrop for IndexBufferInner {
    #[inline]
    fn drop(&mut self, engine: &mut Engine) {
        unsafe {
            filament_Engine_destroy5(engine.native_mut(), self.native.as_ptr());
        }
    }
}
