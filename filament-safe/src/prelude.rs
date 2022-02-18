pub trait NativeHandle<N> {
  fn native(&self) -> *const N;
  fn native_mut(&mut self) -> *mut N;
}
