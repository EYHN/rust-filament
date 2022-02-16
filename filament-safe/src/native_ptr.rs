pub struct NativePtr<NativeType> {
    pub(crate) ptr: *mut NativeType,
}

impl<NativeType> NativePtr<NativeType> {
    pub fn new(ptr: *mut NativeType) -> Self {
        Self { ptr }
    }
}

impl<NativeType> Into<NativePtr<NativeType>> for *mut NativeType {
    fn into(self) -> NativePtr<NativeType> {
        NativePtr::new(self)
    }
}

pub trait NativePtrBase<NativeType> {
    fn native_ptr_mut(&mut self) -> &mut NativePtr<NativeType>;
}