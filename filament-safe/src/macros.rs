#[macro_export]
macro_rules! impl_ptr_functions {
    ($ty:ident, $ptr:path) => {
        impl $ty {
            pub(crate) fn handle(&mut self) -> Arc<*mut $ptr> {
                self.ptr.clone()
            }
            pub(crate) fn as_raw_ptr(&self) -> *mut $ptr {
                *self.ptr
            }
            pub(crate) fn as_raw_mut(&mut self) -> &mut $ptr {
                unsafe { &mut **self.ptr }
            }
            pub(crate) fn as_raw_ref(&self) -> &$ptr {
                unsafe { &**self.ptr }
            }
        }
    };
}
