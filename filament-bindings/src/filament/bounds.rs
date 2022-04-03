use crate::{
    bindgen,
    math::{Float3, Mat4f},
};

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

    // Applies an affine transformation to the AABB.
    pub fn transform(&self, mat: Mat4f) -> Aabb {
        let translation = Float3::new(mat.get(3, 0), mat.get(3, 1), mat.get(3, 2));
        let upper_left = mat.upper_left();
        let mut result = Aabb {
            min: translation,
            max: translation,
        };
        for col in 0..3 {
            for row in 0..3 {
                let a = upper_left.get(col, row) * self.min[col];
                let b = upper_left.get(col, row) * self.max[col];
                result.min[row] += if a < b { a } else { b };
                result.max[row] += if a < b { b } else { a };
            }
        }
        return result;
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
