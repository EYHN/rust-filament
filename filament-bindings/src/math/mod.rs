use crate::bindgen;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vec2<T>(pub T, pub T);

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Vec4<T>(pub T, pub T, pub T, pub T);

impl<T> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self(x, y, z, w)
    }
}

macro_rules! math_vec {
    ($vt:ident, $it:ident, $n:ident, $na:ident) => {
        pub type $n = $vt<$it>;
        impl $n {
            #[allow(dead_code)]
            pub(crate) fn native_ptr(&self) -> *const bindgen::$na {
                self as *const Self as *const _
            }

            #[allow(dead_code)]
            pub(crate) fn native_owned(self) -> bindgen::$na {
                unsafe { core::mem::transmute(self) }
            }
        }
    };
}

math_vec!(Vec2, f64, Double2, filament_math_double2);
math_vec!(Vec2, f32, Float2, filament_math_float2);
math_vec!(Vec2, i32, Int2, filament_math_int2);
math_vec!(Vec2, u32, Uint2, filament_math_uint2);
math_vec!(Vec2, i16, Short2, filament_math_short2);
math_vec!(Vec2, bool, Bool2, filament_math_bool2);

math_vec!(Vec3, f64, Double3, filament_math_double3);
math_vec!(Vec3, f32, Float3, filament_math_float3);
math_vec!(Vec3, i32, Int3, filament_math_int3);
math_vec!(Vec3, u32, Uint3, filament_math_uint3);
math_vec!(Vec3, i16, Short3, filament_math_short3);
math_vec!(Vec3, bool, Bool3, filament_math_bool3);

math_vec!(Vec4, f64, Double4, filament_math_double4);
math_vec!(Vec4, f32, Float4, filament_math_float4);
math_vec!(Vec4, i32, Int4, filament_math_int4);
math_vec!(Vec4, u32, Uint4, filament_math_uint4);
math_vec!(Vec4, i16, Short4, filament_math_short4);
math_vec!(Vec4, bool, Bool4, filament_math_bool4);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Mat4f {
    mat: [f32; 16],
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Mat4 {
    mat: [f64; 16],
}

impl Mat4f {
    #[allow(dead_code)]
    pub(crate) fn native(&self) -> *const bindgen::filament_math_mat4f {
        self as *const Self as *const _
    }
}

impl Mat4 {
    #[allow(dead_code)]
    pub(crate) fn native(&self) -> *const bindgen::filament_math_mat4 {
        self as *const Self as *const _
    }
}
