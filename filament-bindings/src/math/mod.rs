use crate::bindgen;

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Vec2<T>(pub T, pub T);

impl<T> Vec2<T> {
    const COMPONENTS: usize = 2;
    pub fn new(x: T, y: T) -> Self {
        Self(x, y)
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T> Vec3<T> {
    const COMPONENTS: usize = 3;
    pub fn new(x: T, y: T, z: T) -> Self {
        Self(x, y, z)
    }
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct Vec4<T>(pub T, pub T, pub T, pub T);

impl<T> Vec4<T> {
    const COMPONENTS: usize = 4;
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
            pub(crate) fn native_ptr_mut(&mut self) -> *mut bindgen::$na {
                self as *mut Self as *mut _
            }

            #[allow(dead_code)]
            pub(crate) fn native_owned(self) -> bindgen::$na {
                unsafe { core::mem::transmute(self) }
            }

            #[allow(dead_code)]
            pub(crate) fn from_native(native: bindgen::$na) -> Self {
                unsafe { core::mem::transmute(native) }
            }
        }
        impl From<[$it; $vt::<$it>::COMPONENTS]> for $n {
            fn from(arr: [$it; $vt::<$it>::COMPONENTS]) -> Self {
                unsafe { core::mem::transmute(arr) }
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
pub struct Mat4f(pub [f32; 16]);

impl Mat4f {
    const COMPONENTS: usize = 16;
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Mat4(pub [f64; 16]);

impl Mat4 {
    const COMPONENTS: usize = 16;
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Mat3(pub [f64; 9]);

impl Mat3 {
    const COMPONENTS: usize = 9;
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Mat3f(pub [f32; 9]);

impl Mat3f {
    const COMPONENTS: usize = 9;
}

macro_rules! impl_mat {
    ($t: ident, $nt:ident, $it:ident) => {
        impl $t {
            #[allow(dead_code)]
            pub(crate) fn native_ptr(&self) -> *const bindgen::$nt {
                self as *const Self as *const _
            }

            #[allow(dead_code)]
            pub(crate) fn from_native(native: bindgen::$nt) -> Self {
                unsafe { core::mem::transmute(native) }
            }
        }
        impl From<[$it; $t::COMPONENTS]> for $t {
            fn from(mat: [$it; $t::COMPONENTS]) -> Self {
                Self(mat)
            }
        }
    };
}

impl_mat!(Mat4f, filament_math_mat4f, f32);
impl_mat!(Mat4, filament_math_mat4, f64);
impl_mat!(Mat3f, filament_math_mat3f, f32);
impl_mat!(Mat3, filament_math_mat3, f64);
