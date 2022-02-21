use std::{ptr, rc::Rc};

use filament_bindings::{
    filament_Engine_destroy5, filament_IndexBuffer, filament_IndexBuffer_Builder,
    filament_IndexBuffer_IndexType_UINT, filament_IndexBuffer_IndexType_USHORT,
    filament_IndexBuffer_getIndexCount, filament_IndexBuffer_setBuffer,
};
use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{backend::BufferDescriptor, prelude::NativeHandle};

use super::Engine;

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
        IndexBuffer::try_from_native(engine.clone(), unsafe {
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

#[derive(Clone)]
pub struct IndexBuffer {
    native: Rc<ptr::NonNull<filament_IndexBuffer>>,
    engine: Engine,
}

impl NativeHandle<filament_IndexBuffer> for IndexBuffer {
    #[inline]
    fn native(&self) -> *const filament_IndexBuffer {
        self.native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_IndexBuffer {
        self.native.as_ptr()
    }
}

impl IndexBuffer {
    #[inline]
    pub(crate) fn try_from_native(
        engine: Engine,
        native: *mut filament_IndexBuffer,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self {
            native: Rc::new(ptr),
            engine,
        })
    }
}

impl IndexBuffer {
    pub fn get_index_count(&self) -> usize {
        unsafe { filament_IndexBuffer_getIndexCount(self.native()) as usize }
    }

    pub fn set_buffer<T>(&mut self, buffer: BufferDescriptor<T>, byte_offset: u32) -> &mut Self {
        unsafe {
            filament_IndexBuffer_setBuffer(
                self.native_mut(),
                self.engine.native_mut(),
                &mut buffer.into_native(),
                byte_offset,
            )
        };
        self
    }
}

impl Drop for IndexBuffer {
    #[inline]
    fn drop(&mut self) {
        if let Some(_) = Rc::get_mut(&mut self.native) {
            unsafe { filament_Engine_destroy5(self.engine.native_mut(), self.native()) };
        }
    }
}
