use std::ptr;

use filament_bindings::{
    filament_backend_BufferDescriptor, filament_backend_PixelBufferDescriptor,
    filament_backend_PixelBufferDescriptor__bindgen_ty_1,
    filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1,
};

use super::{PixelDataFormat, PixelDataType};

pub struct BufferDescriptor<T> {
    data: Vec<T>,
    callback: Box<Box<dyn FnOnce(&mut Vec<u8>)>>,
}

pub struct PixelBufferDescriptor<T> {
    buffer: BufferDescriptor<T>,
    format: PixelDataFormat,
    datatype: PixelDataType,
    alignment: u8,
    left: u32,
    top: u32,
    stride: u32,
}

impl<T> BufferDescriptor<T> {
    pub fn new(data: Vec<T>) -> Self {
        BufferDescriptor {
            data,
            callback: Box::new(Box::new(|_| {})),
        }
    }

    pub fn new_callback(data: Vec<T>, callback: impl FnOnce(&mut Vec<u8>) + 'static) -> Self {
        BufferDescriptor {
            data,
            callback: Box::new(Box::new(callback)),
        }
    }

    pub fn into_native(mut self) -> filament_backend_BufferDescriptor {
        let user = Box::into_raw(self.callback);
        let desc = filament_backend_BufferDescriptor {
            buffer: self.data.as_mut_ptr() as *mut _,
            size: (self.data.len() * std::mem::size_of::<T>())
                .try_into()
                .unwrap(),
            mCallback: Some(buffer_descriptor_callback),
            mUser: user as *mut _,
            mHandler: ptr::null_mut(),
        };
        std::mem::forget(self.data);
        desc
    }
}

impl<T> PixelBufferDescriptor<T> {
    pub fn new(
        data: Vec<T>,
        format: PixelDataFormat,
        datatype: PixelDataType,
    ) -> Self {
        PixelBufferDescriptor {
            buffer: BufferDescriptor::new(data),
            format,
            datatype,
            alignment: 1,
            left: 0,
            top: 0,
            stride: 0,
        }
    }

    pub fn new_callback(
        data: Vec<T>,
        format: PixelDataFormat,
        datatype: PixelDataType,
        callback: impl FnOnce(&mut Vec<u8>) + 'static,
    ) -> Self {
        PixelBufferDescriptor {
            buffer: BufferDescriptor::new_callback(data, callback),
            format,
            datatype,
            alignment: 1,
            left: 0,
            top: 0,
            stride: 0,
        }
    }

    pub fn into_native(self) -> filament_backend_PixelBufferDescriptor {
        filament_backend_PixelBufferDescriptor {
            _base: self.buffer.into_native(),
            left: self.left,
            top: self.top,
            __bindgen_anon_1: filament_backend_PixelBufferDescriptor__bindgen_ty_1 {
                __bindgen_anon_1:
                    filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1 {
                        stride: self.stride,
                        format: self.format.into(),
                    },
            },
            _bitfield_1: filament_backend_PixelBufferDescriptor::new_bitfield_1(
                self.datatype.into(),
                self.alignment,
            ),
            ..Default::default()
        }
    }
}

unsafe extern "C" fn buffer_descriptor_callback(
    ptr: *mut std::ffi::c_void,
    size: u64,
    user: *mut std::ffi::c_void,
) {
    let mut buffer: Vec<u8> = Vec::from_raw_parts(ptr as *mut _, size as usize, size as usize);

    if !user.is_null() {
        let user_fn: Box<Box<dyn FnOnce(&mut Vec<u8>)>> = Box::from_raw(user as *mut _);
        user_fn(&mut buffer);
    }

    std::mem::drop(buffer);
}
