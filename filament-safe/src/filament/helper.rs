use super::Engine;

pub struct EngineManaged<'a, NativeType> where NativeType: EngineDestroy {
  pub(crate) engine: &'a mut Engine,
  pub ptr: *mut NativeType,
}

impl<'a, NativeType> EngineManaged<'a, NativeType> where NativeType: EngineDestroy {
  pub fn from_ptr(engine: &'a mut Engine, ptr: *mut NativeType) -> Self {
      Self { engine, ptr }
  }
}

pub trait EngineDestroy {
    fn destory(p: *const Self, engine: &mut Engine) -> bool;
}

impl<NativeType> Drop for EngineManaged<'_, NativeType> where NativeType: EngineDestroy {
  fn drop(&mut self) {
    NativeType::destory(self.ptr, self.engine);
  }
}
