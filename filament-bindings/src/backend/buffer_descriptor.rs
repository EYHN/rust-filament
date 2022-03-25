use std::ptr;

use super::{PixelDataFormat, PixelDataType};

pub struct BufferDescriptor<T> {
    data: *const T,
    size_in_bytes: usize,
    callback: Box<Box<dyn FnOnce()>>,
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

impl<T: 'static> BufferDescriptor<T> {
    #[inline]
    pub unsafe fn new(mut data: Vec<T>) -> Self {
        BufferDescriptor {
            data: data.as_mut_ptr(),
            size_in_bytes: data.len() * std::mem::size_of::<T>(),
            callback: Box::new(Box::new(move || std::mem::drop(data))),
        }
    }

    #[inline]
    pub unsafe fn from_raw_ptr(data: *const T, size_in_bytes: usize) -> Self {
        BufferDescriptor {
            data,
            size_in_bytes,
            callback: Box::new(Box::new(|| {})),
        }
    }

    #[inline]
    pub unsafe fn new_callback(mut data: Vec<T>, callback: impl FnOnce(Vec<T>) + 'static) -> Self {
        BufferDescriptor {
            data: data.as_mut_ptr(),
            size_in_bytes: data.len() * std::mem::size_of::<T>(),
            callback: Box::new(Box::new(move || callback(data))),
        }
    }

    #[inline]
    pub unsafe fn from_raw_ptr_callback(
        data: *const T,
        size_in_bytes: usize,
        callback: impl FnOnce(*const T) + 'static,
    ) -> Self {
        BufferDescriptor {
            data,
            size_in_bytes,
            callback: Box::new(Box::new(move || callback(data))),
        }
    }

    #[inline]
    pub unsafe fn into_native(self) -> crate::bindgen::filament_backend_BufferDescriptor {
        let user = Box::into_raw(self.callback);
        let desc = crate::bindgen::filament_backend_BufferDescriptor {
            buffer: self.data as *mut _,
            size: self.size_in_bytes,
            mCallback: Some(buffer_descriptor_callback),
            mUser: user as *mut _,
            mHandler: ptr::null_mut(),
        };
        std::mem::forget(self.data);
        desc
    }
}

impl<T: 'static> PixelBufferDescriptor<T> {
    #[inline]
    pub unsafe fn new(data: Vec<T>, format: PixelDataFormat, datatype: PixelDataType) -> Self {
        PixelBufferDescriptor {
            buffer: BufferDescriptor::<T>::new(data),
            format,
            datatype,
            alignment: 1,
            left: 0,
            top: 0,
            stride: 0,
        }
    }

    #[inline]
    pub unsafe fn new_callback(
        data: Vec<T>,
        format: PixelDataFormat,
        datatype: PixelDataType,
        callback: impl FnOnce(Vec<T>) + 'static,
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

    #[inline]
    pub unsafe fn from_raw_ptr(
        data: *const T,
        size_in_bytes: usize,
        format: PixelDataFormat,
        datatype: PixelDataType,
    ) -> Self {
        PixelBufferDescriptor {
            buffer: BufferDescriptor::<T>::from_raw_ptr(data, size_in_bytes),
            format,
            datatype,
            alignment: 1,
            left: 0,
            top: 0,
            stride: 0,
        }
    }

    #[inline]
    pub unsafe fn from_raw_ptr_callback(
        data: *const T,
        size_in_bytes: usize,
        format: PixelDataFormat,
        datatype: PixelDataType,
        callback: impl FnOnce(*const T) + 'static,
    ) -> Self {
        PixelBufferDescriptor {
            buffer: BufferDescriptor::from_raw_ptr_callback(data, size_in_bytes, callback),
            format,
            datatype,
            alignment: 1,
            left: 0,
            top: 0,
            stride: 0,
        }
    }

    #[inline]
    pub unsafe fn into_native(self) -> crate::bindgen::filament_backend_PixelBufferDescriptor {
        crate::bindgen::filament_backend_PixelBufferDescriptor {
            _base: self.buffer.into_native(),
            left: self.left,
            top: self.top,
            __bindgen_anon_1: crate::bindgen::filament_backend_PixelBufferDescriptor__bindgen_ty_1 {
                __bindgen_anon_1:
                    crate::bindgen::filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1 {
                        stride: self.stride,
                        format: self.format.into(),
                    },
            },
            _bitfield_1: crate::bindgen::filament_backend_PixelBufferDescriptor::new_bitfield_1(
                self.datatype.into(),
                self.alignment,
            ),
            ..Default::default()
        }
    }
}

unsafe extern "C" fn buffer_descriptor_callback(
    _ptr: *mut std::ffi::c_void,
    _size: usize,
    user: *mut std::ffi::c_void,
) {
    if !user.is_null() {
        let user_fn: Box<Box<dyn FnOnce()>> = Box::from_raw(user as *mut _);
        user_fn();
    }
}
