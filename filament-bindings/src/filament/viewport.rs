use std::mem;

use crate::bindgen;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Viewport {
    pub left: i32,
    pub bottom: i32,
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub fn new_from_ptr(ptr: *const bindgen::filament_Viewport) -> Viewport {
        unsafe {
            Viewport {
                left: (*ptr)._base.left,
                bottom: (*ptr)._base.bottom,
                width: (*ptr)._base.width,
                height: (*ptr)._base.height,
            }
        }
    }

    pub fn as_native(&self) -> &bindgen::filament_Viewport {
        unsafe { mem::transmute(self) }
    }
}
