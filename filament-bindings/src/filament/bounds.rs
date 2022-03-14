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

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Aabb {
    pub min: Float3,
    pub max: Float3,
}

impl Aabb {
    pub fn center(&self) -> Float3 {
        (self.max + self.min) * 0.5
    }

    pub fn extent(&self) -> Float3 {
        (self.max - self.min) * 0.5
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Self {
            min: f32::MAX.into(),
            max: f32::MIN.into(),
        }
    }
}
