use std::ptr;

use crate::bindgen;

pub struct TransformManager {
    native: ptr::NonNull<bindgen::filament_TransformManager>,
}

impl TransformManager {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_TransformManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_TransformManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_TransformManager) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }
}

// TODO
