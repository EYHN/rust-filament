use std::ptr;

use crate::bindgen;

pub struct Fence {
    native: ptr::NonNull<bindgen::filament_Fence>,
}

impl Fence {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_Fence {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_Fence {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_Fence) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }
}

// TODO