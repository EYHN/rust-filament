use std::mem;

use filament_bindings::filament_Viewport;

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Viewport {
    pub left: i32,
    pub bottom: i32,
    pub width: u32,
    pub height: u32,
}

impl Viewport {
  pub(crate) fn as_native(&self) -> &filament_Viewport {
    unsafe {
      mem::transmute(self)
    }
  }
}