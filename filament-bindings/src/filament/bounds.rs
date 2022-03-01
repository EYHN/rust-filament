use crate::{bindgen, math::Float3};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Bounds {
    pub center: Float3,
    pub half_extent: Float3,
}

impl Bounds {
    pub(crate) fn native_ptr(&self) -> *const bindgen::filament_Box {
        self as *const Self as *const _
    }
}
