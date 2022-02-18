use std::{ptr, rc::Rc};

use filament_bindings::{filament_View, filament_Engine_destroy19};

use crate::prelude::NativeHandle;

use super::Engine;

pub struct View(Rc<ptr::NonNull<filament_View>>, Engine);

impl NativeHandle<filament_View> for View {
  #[inline]
  fn native(&self) -> *const filament_View {
      self.0.as_ptr()
  }

  #[inline]
  fn native_mut(&mut self) -> *mut filament_View {
      self.0.as_ptr()
  }
}

impl Drop for View {
  #[inline]
  fn drop(&mut self) {
      if let Some(_) = Rc::get_mut(&mut self.0) {
          unsafe { filament_Engine_destroy19(self.1.native_mut(), self.native()) };
      }
  }
}
