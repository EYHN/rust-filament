use crate::bindgen;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2<T: Copy> {
    pub vec: [T; 2],
}

impl<T: Copy> Vec2<T> {
    const COMPONENTS: usize = 2;
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { vec: [x, y] }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3<T: Copy> {
    pub vec: [T; 3],
}

impl<T: Copy> Vec3<T> {
    const COMPONENTS: usize = 3;
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { vec: [x, y, z] }
    }

    #[inline]
    pub fn xy(&self) -> Vec2<T> {
        Vec2::new(self.vec[0], self.vec[1])
    }
}

impl<T: Copy + std::ops::Mul<Output = T> + std::ops::Sub<Output = T>> Vec3<T> {
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            vec: [
                self.vec[1] * other.vec[2] - self.vec[2] * other.vec[1],
                self.vec[2] * other.vec[0] - self.vec[0] * other.vec[2],
                self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0],
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec4<T: Copy> {
    pub vec: [T; 4],
}

impl<T: Copy> Vec4<T> {
    const COMPONENTS: usize = 4;
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { vec: [x, y, z, w] }
    }

    #[inline]
    pub fn from_vec3(vec3: Vec3<T>, w: T) -> Self {
        Self {
            vec: [vec3.vec[0], vec3.vec[1], vec3.vec[2], w],
        }
    }

    #[inline]
    pub fn xyz(&self) -> Vec3<T> {
        Vec3::new(self.vec[0], self.vec[1], self.vec[2])
    }
}

impl<T: Copy> From<Quaternion<T>> for Vec4<T> {
    fn from(s: Quaternion<T>) -> Self {
        Self { vec: s.vec }
    }
}

macro_rules! impl_named_vec {
    ($vt:ident, $it:ty, $n:ident, $na:ident) => {
        pub type $n = $vt<$it>;
        impl $n {
            #[allow(dead_code)]
            #[inline]
            pub(crate) fn native_ptr(&self) -> *const bindgen::$na {
                self as *const Self as *const _
            }

            #[allow(dead_code)]
            #[inline]
            pub(crate) fn native_ptr_mut(&mut self) -> *mut bindgen::$na {
                self as *mut Self as *mut _
            }

            #[allow(dead_code)]
            #[inline]
            pub(crate) fn native_owned(self) -> bindgen::$na {
                unsafe { core::mem::transmute(self) }
            }

            #[allow(dead_code)]
            #[inline]
            pub(crate) fn from_native(native: bindgen::$na) -> Self {
                unsafe { core::mem::transmute(native) }
            }
        }
        impl From<[$it; $vt::<$it>::COMPONENTS]> for $n {
            #[inline]
            fn from(arr: [$it; $vt::<$it>::COMPONENTS]) -> Self {
                unsafe { core::mem::transmute(arr) }
            }
        }
        impl From<$it> for $n {
            #[inline]
            fn from(v: $it) -> Self {
                let mut vec = Self::default();
                for a in 0..$vt::<$it>::COMPONENTS {
                    vec.vec[a] = v;
                }
                vec
            }
        }
        impl std::ops::Index<usize> for $n {
            type Output = $it;
            #[inline]
            fn index(&self, index: usize) -> &$it {
                &self.vec[index]
            }
        }
        impl std::ops::IndexMut<usize> for $n {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut $it {
                &mut self.vec[index]
            }
        }
    };
}

macro_rules! impl_named_vec_ops {
    ($vt:ident, $it:ty, $n:ident) => {
        impl $n {
            #[inline]
            pub fn max(&self, v: $n) -> $n {
                let mut r = $n::default();
                for a in 0..$vt::<$it>::COMPONENTS {
                    r.vec[a] = <$it>::max(self.vec[a], v.vec[a])
                }
                r
            }

            #[inline]
            pub fn min(&self, v: $n) -> $n {
                let mut r = $n::default();
                for a in 0..$vt::<$it>::COMPONENTS {
                    r.vec[a] = <$it>::min(self.vec[a], v.vec[a])
                }
                r
            }
        }
        impl std::ops::AddAssign<$n> for $n {
            #[inline]
            fn add_assign(&mut self, v: $n) {
                for a in 0..$vt::<$it>::COMPONENTS {
                    self.vec[a] += v.vec[a];
                }
            }
        }
        impl std::ops::AddAssign<$it> for $n {
            #[inline]
            fn add_assign(&mut self, v: $it) {
                for a in 0..$vt::<$it>::COMPONENTS {
                    self.vec[a] += v;
                }
            }
        }
        impl std::ops::Add<$n> for $n {
            type Output = $n;
            #[inline]
            fn add(mut self, v: $n) -> $n {
                self += v;
                self
            }
        }
        impl std::ops::Add<$it> for $n {
            type Output = $n;
            #[inline]
            fn add(mut self, v: $it) -> $n {
                self += v;
                self
            }
        }

        impl std::ops::SubAssign<$n> for $n {
            #[inline]
            fn sub_assign(&mut self, v: $n) {
                for a in 0..$vt::<$it>::COMPONENTS {
                    self.vec[a] -= v.vec[a];
                }
            }
        }
        impl std::ops::SubAssign<$it> for $n {
            #[inline]
            fn sub_assign(&mut self, v: $it) {
                for a in 0..$vt::<$it>::COMPONENTS {
                    self.vec[a] -= v;
                }
            }
        }
        impl std::ops::Sub<$n> for $n {
            type Output = $n;
            #[inline]
            fn sub(mut self, v: $n) -> $n {
                self -= v;
                self
            }
        }
        impl std::ops::Sub<$it> for $n {
            type Output = $n;
            #[inline]
            fn sub(mut self, v: $it) -> $n {
                self -= v;
                self
            }
        }

        impl std::ops::MulAssign<$n> for $n {
            #[inline]
            fn mul_assign(&mut self, v: $n) {
                for a in 0..$vt::<$it>::COMPONENTS {
                    self.vec[a] *= v.vec[a];
                }
            }
        }
        impl std::ops::MulAssign<$it> for $n {
            #[inline]
            fn mul_assign(&mut self, v: $it) {
                for a in 0..$vt::<$it>::COMPONENTS {
                    self.vec[a] *= v;
                }
            }
        }
        impl std::ops::Mul<$n> for $n {
            type Output = $n;
            #[inline]
            fn mul(mut self, v: $n) -> $n {
                self *= v;
                self
            }
        }
        impl std::ops::Mul<$it> for $n {
            type Output = $n;
            #[inline]
            fn mul(mut self, v: $it) -> $n {
                self *= v;
                self
            }
        }

        impl std::ops::DivAssign<$n> for $n {
            #[inline]
            fn div_assign(&mut self, v: $n) {
                for a in 0..$vt::<$it>::COMPONENTS {
                    self.vec[a] /= v.vec[a];
                }
            }
        }
        impl std::ops::DivAssign<$it> for $n {
            #[inline]
            fn div_assign(&mut self, v: $it) {
                for a in 0..$vt::<$it>::COMPONENTS {
                    self.vec[a] /= v;
                }
            }
        }
        impl std::ops::Div<$n> for $n {
            type Output = $n;
            #[inline]
            fn div(mut self, v: $n) -> $n {
                self /= v;
                self
            }
        }
        impl std::ops::Div<$it> for $n {
            type Output = $n;
            #[inline]
            fn div(mut self, v: $it) -> $n {
                self /= v;
                self
            }
        }
    };
}

macro_rules! impl_named_vec_float_ops {
    ($vt:ident, $it:ty, $n:ident) => {
        impl $n {
            #[inline]
            #[allow(dead_code)]
            pub fn dot(&self, rv: &Self) -> $it {
                let mut r = <$it>::default();
                for a in 0..$vt::<$it>::COMPONENTS {
                    r += self[a] * rv[a];
                }
                r
            }

            #[inline]
            #[allow(dead_code)]
            pub fn norm(&self) -> $it {
                self.dot(&self).sqrt()
            }

            #[inline]
            #[allow(dead_code)]
            pub fn length(&self) -> $it {
                self.norm()
            }

            #[inline]
            #[allow(dead_code)]
            pub fn norm2(&self) -> $it {
                self.dot(&self)
            }

            #[inline]
            #[allow(dead_code)]
            pub fn length2(&self) -> $it {
                self.norm2()
            }

            #[inline]
            #[allow(dead_code)]
            pub fn normalize(&self) -> Self {
                self.clone() * (1 as $it / self.length())
            }

            #[inline]
            #[allow(dead_code)]
            pub fn pack_unorm16(&self) -> $vt<u16> {
                let mut r = $vt::<u16>::default();
                for a in 0..$vt::<$it>::COMPONENTS {
                    r[a] = <$it>::round(<$it>::clamp(self[a], 0.0, 1.0) * 65535.0) as u16
                }
                r
            }

            #[inline]
            #[allow(dead_code)]
            pub fn pack_snorm16(&self) -> $vt<i16> {
                let mut r = $vt::<i16>::default();
                for a in 0..$vt::<$it>::COMPONENTS {
                    r[a] = <$it>::round(<$it>::clamp(self[a], -1.0, 1.0) * 32767.0) as i16
                }
                r
            }

            #[inline]
            #[allow(dead_code)]
            pub fn unpack_unorm16(v: &$vt<u16>) -> Self {
                let mut r = Self::default();
                for a in 0..$vt::<$it>::COMPONENTS {
                    r[a] = v[a] as $it / 65535.0
                }
                r
            }

            #[inline]
            #[allow(dead_code)]
            pub fn unpack_snorm16(v: &$vt<i16>) -> Self {
                let mut r = Self::default();
                for a in 0..$vt::<$it>::COMPONENTS {
                    r[a] = <$it>::clamp(v[a] as $it / 32767.0, -1.0, 1.0)
                }
                r
            }
        }
    };
}

impl_named_vec!(Vec2, f64, Double2, filament_math_double2);
impl_named_vec_ops!(Vec2, f64, Double2);
impl_named_vec_float_ops!(Vec2, f64, Double2);
impl_named_vec!(Vec2, f32, Float2, filament_math_float2);
impl_named_vec_ops!(Vec2, f32, Float2);
impl_named_vec_float_ops!(Vec2, f32, Float2);
impl_named_vec!(Vec2, half::f16, Half2, filament_math_half2);
impl_named_vec_ops!(Vec2, half::f16, Half2);
impl_named_vec!(Vec2, i32, Int2, filament_math_int2);
impl_named_vec_ops!(Vec2, i32, Int2);
impl_named_vec!(Vec2, u32, Uint2, filament_math_uint2);
impl_named_vec_ops!(Vec2, u32, Uint2);
impl_named_vec!(Vec2, i16, Short2, filament_math_short2);
impl_named_vec_ops!(Vec2, i16, Short2);
impl_named_vec!(Vec2, u16, Ushort2, filament_math_ushort2);
impl_named_vec_ops!(Vec2, u16, Ushort2);
impl_named_vec!(Vec2, bool, Bool2, filament_math_bool2);

impl_named_vec!(Vec3, f64, Double3, filament_math_double3);
impl_named_vec_ops!(Vec3, f64, Double3);
impl_named_vec_float_ops!(Vec3, f64, Double3);
impl_named_vec!(Vec3, f32, Float3, filament_math_float3);
impl_named_vec_ops!(Vec3, f32, Float3);
impl_named_vec_float_ops!(Vec3, f32, Float3);
impl_named_vec!(Vec3, half::f16, Half3, filament_math_half3);
impl_named_vec_ops!(Vec3, half::f16, Half3);
impl_named_vec!(Vec3, i32, Int3, filament_math_int3);
impl_named_vec_ops!(Vec3, i32, Int3);
impl_named_vec!(Vec3, u32, Uint3, filament_math_uint3);
impl_named_vec_ops!(Vec3, u32, Uint3);
impl_named_vec!(Vec3, i16, Short3, filament_math_short3);
impl_named_vec_ops!(Vec3, i16, Short3);
impl_named_vec!(Vec3, u16, Ushort3, filament_math_ushort3);
impl_named_vec_ops!(Vec3, u16, Ushort3);
impl_named_vec!(Vec3, bool, Bool3, filament_math_bool3);

impl_named_vec!(Vec4, f64, Double4, filament_math_double4);
impl_named_vec_ops!(Vec4, f64, Double4);
impl_named_vec_float_ops!(Vec4, f64, Double4);
impl_named_vec!(Vec4, f32, Float4, filament_math_float4);
impl_named_vec_ops!(Vec4, f32, Float4);
impl_named_vec_float_ops!(Vec4, f32, Float4);
impl_named_vec!(Vec4, half::f16, Half4, filament_math_half4);
impl_named_vec_ops!(Vec4, half::f16, Half4);
impl_named_vec!(Vec4, i32, Int4, filament_math_int4);
impl_named_vec_ops!(Vec4, i32, Int4);
impl_named_vec!(Vec4, u32, Uint4, filament_math_uint4);
impl_named_vec_ops!(Vec4, u32, Uint4);
impl_named_vec!(Vec4, i16, Short4, filament_math_short4);
impl_named_vec_ops!(Vec4, i16, Short4);
impl_named_vec!(Vec4, u16, Ushort4, filament_math_ushort4);
impl_named_vec_ops!(Vec4, u16, Ushort4);
impl_named_vec!(Vec4, bool, Bool4, filament_math_bool4);

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4f(pub [f32; 16]);

impl Mat4f {
    const COMPONENTS: usize = 16;
    const ROWS: usize = 4;
    const COLUMNS: usize = 4;

    // Constructs a 3x3 matrix from the upper-left corner of this 4x4 matrix
    pub fn upper_left(&self) -> Mat3f {
        Mat3f([
            self.0[0], self.0[1], self.0[2], self.0[4], self.0[5], self.0[6], self.0[8], self.0[9],
            self.0[10],
        ])
    }

    pub fn scaling(scale: Float3) -> Self {
        Self([
            scale[0], 0.0, 0.0, 0.0, 0.0, scale[1], 0.0, 0.0, 0.0, 0.0, scale[2], 0.0, 0.0, 0.0,
            0.0, 1.0,
        ])
    }

    pub fn translation(t: Float3) -> Self {
        Self([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, t[0], t[1], t[2], 1.0,
        ])
    }

    pub fn look_at(eye: &Float3, center: &Float3, up: &Float3) -> Self {
        let z_axis = (*center - *eye).normalize();
        let mut norm_up = up.normalize();
        if z_axis.dot(&norm_up).abs() > 0.999 {
            // Fix up vector if we're degenerate (looking straight up, basically)
            norm_up = Float3::new(norm_up[2], norm_up[0], norm_up[1]);
        }
        let x_axis = z_axis.cross(&norm_up).normalize();
        let y_axis = x_axis.cross(&z_axis);
        Self([
            x_axis[0], x_axis[1], x_axis[2], 0.0, y_axis[0], y_axis[1], y_axis[2], 0.0, -z_axis[0],
            -z_axis[1], -z_axis[2], 0.0, eye[0], eye[1], eye[2], 1.0,
        ])
    }
}

impl Default for Mat4f {
    fn default() -> Self {
        Self([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }
}

impl From<Mat4> for Mat4f {
    fn from(m: Mat4) -> Self {
        Self([
            m.0[0] as f32,
            m.0[1] as f32,
            m.0[2] as f32,
            m.0[3] as f32,
            m.0[4] as f32,
            m.0[5] as f32,
            m.0[6] as f32,
            m.0[7] as f32,
            m.0[8] as f32,
            m.0[9] as f32,
            m.0[10] as f32,
            m.0[11] as f32,
            m.0[12] as f32,
            m.0[13] as f32,
            m.0[14] as f32,
            m.0[15] as f32,
        ])
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4(pub [f64; 16]);

impl Mat4 {
    const COMPONENTS: usize = 16;
    const ROWS: usize = 4;
    const COLUMNS: usize = 4;

    // Constructs a 3x3 matrix from the upper-left corner of this 4x4 matrix
    pub fn upper_left(&self) -> Mat3 {
        Mat3([
            self.0[0], self.0[1], self.0[2], self.0[4], self.0[5], self.0[6], self.0[8], self.0[9],
            self.0[10],
        ])
    }

    pub fn scaling(scale: Double3) -> Self {
        Self([
            scale[0], 0.0, 0.0, 0.0, 0.0, scale[1], 0.0, 0.0, 0.0, 0.0, scale[2], 0.0, 0.0, 0.0,
            0.0, 1.0,
        ])
    }

    pub fn translation(t: Double3) -> Self {
        Self([
            1.0, 0.0, 0.0, t[0], 0.0, 1.0, 0.0, t[1], 0.0, 0.0, 1.0, t[2], 0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn look_at(eye: &Double3, center: &Double3, up: &Double3) -> Self {
        let z_axis = (*center - *eye).normalize();
        let mut norm_up = up.normalize();
        if z_axis.dot(&norm_up).abs() > 0.999 {
            // Fix up vector if we're degenerate (looking straight up, basically)
            norm_up = Double3::new(norm_up[2], norm_up[0], norm_up[1]);
        }
        let x_axis = z_axis.cross(&norm_up).normalize();
        let y_axis = x_axis.cross(&z_axis);
        Self([
            x_axis[0], y_axis[0], -z_axis[0], eye[0], x_axis[1], y_axis[1], -z_axis[1], eye[1],
            x_axis[2], y_axis[2], -z_axis[2], eye[2], 0.0, 0.0, 0.0, 1.0,
        ])
    }
}

impl Default for Mat4 {
    fn default() -> Self {
        Self([
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ])
    }
}

impl From<Mat4f> for Mat4 {
    fn from(m: Mat4f) -> Self {
        Self([
            m.0[0] as f64,
            m.0[1] as f64,
            m.0[2] as f64,
            m.0[3] as f64,
            m.0[4] as f64,
            m.0[5] as f64,
            m.0[6] as f64,
            m.0[7] as f64,
            m.0[8] as f64,
            m.0[9] as f64,
            m.0[10] as f64,
            m.0[11] as f64,
            m.0[12] as f64,
            m.0[13] as f64,
            m.0[14] as f64,
            m.0[15] as f64,
        ])
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3(pub [f64; 9]);

impl Mat3 {
    const COMPONENTS: usize = 9;
    const ROWS: usize = 3;
    const COLUMNS: usize = 3;

    pub unsafe fn pack_tangent_frame(&self) -> Quat {
        let mut result = Quat::default();
        bindgen::helper_filament_math_mat3_pack_tangent_frame(
            self.native_ptr(),
            core::mem::size_of::<i16>(),
            result.native_ptr_mut(),
        );
        result
    }
}

impl Default for Mat3 {
    fn default() -> Self {
        Self([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }
}

impl From<Mat3f> for Mat3 {
    fn from(m: Mat3f) -> Self {
        Self([
            m.0[0] as f64,
            m.0[1] as f64,
            m.0[2] as f64,
            m.0[3] as f64,
            m.0[4] as f64,
            m.0[5] as f64,
            m.0[6] as f64,
            m.0[7] as f64,
            m.0[8] as f64,
        ])
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat3f(pub [f32; 9]);

impl Mat3f {
    const COMPONENTS: usize = 9;
    const ROWS: usize = 3;
    const COLUMNS: usize = 3;

    pub unsafe fn pack_tangent_frame(&self) -> Quatf {
        let mut result = Quatf::default();
        bindgen::helper_filament_math_mat3f_pack_tangent_frame(
            self.native_ptr(),
            core::mem::size_of::<i16>(),
            result.native_ptr_mut(),
        );
        return result;
    }
}

impl Default for Mat3f {
    fn default() -> Self {
        Self([1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }
}

impl From<Mat3> for Mat3f {
    fn from(m: Mat3) -> Self {
        Self([
            m.0[0] as f32,
            m.0[1] as f32,
            m.0[2] as f32,
            m.0[3] as f32,
            m.0[4] as f32,
            m.0[5] as f32,
            m.0[6] as f32,
            m.0[7] as f32,
            m.0[8] as f32,
        ])
    }
}

impl From<(Float3, Float3, Float3)> for Mat3f {
    fn from(m: (Float3, Float3, Float3)) -> Self {
        Self([
            m.0[0] as f32,
            m.0[1] as f32,
            m.0[2] as f32,
            m.1[0] as f32,
            m.1[1] as f32,
            m.1[2] as f32,
            m.2[0] as f32,
            m.2[1] as f32,
            m.2[2] as f32,
        ])
    }
}

macro_rules! impl_mat {
    ($t: ident, $nt:ident, $it:ident) => {
        impl $t {
            #[allow(dead_code)]
            #[inline]
            pub(crate) fn native_ptr(&self) -> *const bindgen::$nt {
                self as *const Self as *const _
            }

            #[allow(dead_code)]
            #[inline]
            pub(crate) fn from_native(native: bindgen::$nt) -> Self {
                unsafe { core::mem::transmute(native) }
            }

            #[allow(dead_code)]
            #[inline]
            pub fn get(&self, column: usize, row: usize) -> $it {
                self.0[column * $t::ROWS + row]
            }

            #[allow(dead_code)]
            #[inline]
            pub fn set(&mut self, column: usize, row: usize, v: $it) {
                self.0[column * $t::ROWS + row] = v
            }

            #[allow(dead_code)]
            pub fn rotation(radian: $it, about: Vec3<$it>) -> Self {
                let mut r = Self::default();
                let c = $it::cos(radian);
                let s = $it::sin(radian);
                if about[0] == 1.0 && about[1] == 0.0 && about[2] == 0.0 {
                    r.set(1, 1, c);
                    r.set(2, 2, c);
                    r.set(1, 2, s);
                    r.set(2, 1, -s);
                } else if about[0] == 0.0 && about[1] == 1.0 && about[2] == 0.0 {
                    r.set(0, 0, c);
                    r.set(2, 2, c);
                    r.set(2, 0, s);
                    r.set(0, 2, -s);
                } else if about[0] == 0.0 && about[1] == 0.0 && about[2] == 1.0 {
                    r.set(0, 0, c);
                    r.set(1, 1, c);
                    r.set(0, 1, s);
                    r.set(1, 0, -s);
                } else {
                    let nabout = about.normalize();
                    let x = nabout[0];
                    let y = nabout[1];
                    let z = nabout[2];
                    let nc = 1.0 - c;
                    let xy = x * y;
                    let yz = y * z;
                    let zx = z * x;
                    let xs = x * s;
                    let ys = y * s;
                    let zs = z * s;
                    r.set(0, 0, x * x * nc + c);
                    r.set(1, 0, xy * nc - zs);
                    r.set(2, 0, zx * nc + ys);

                    r.set(0, 1, xy * nc + zs);
                    r.set(1, 1, y * y * nc + c);
                    r.set(2, 1, yz * nc - xs);

                    r.set(0, 2, zx * nc - ys);
                    r.set(1, 2, yz * nc + xs);
                    r.set(2, 2, z * z * nc + c);

                    // Clamp results to -1, 1.
                    for col in 0..$t::COLUMNS {
                        for row in 0..$t::ROWS {
                            r.set(col, row, r.get(col, row).clamp(-1.0, 1.0));
                        }
                    }
                }
                r
            }

            #[allow(dead_code)]
            pub fn is_close_to(&self, rhs: &$t) -> bool {
                for a in 0..$t::COMPONENTS {
                    if (self.0[a] - rhs.0[a]).abs() > 0.00001 {
                        return false;
                    }
                }
                true
            }
        }
        impl From<[$it; $t::COMPONENTS]> for $t {
            fn from(mat: [$it; $t::COMPONENTS]) -> Self {
                Self(mat)
            }
        }
        impl std::ops::MulAssign<$it> for $t {
            #[inline]
            fn mul_assign(&mut self, v: $it) {
                for a in 0..$t::COMPONENTS {
                    self.0[a] *= v;
                }
            }
        }

        // matrix *= matrix
        impl std::ops::MulAssign<$t> for $t {
            #[inline]
            fn mul_assign(&mut self, rhs: $t) {
                let lhs = self.clone();

                for col in 0..$t::COLUMNS {
                    let rhscol = Vec4::new(
                        rhs.get(col, 0),
                        rhs.get(col, 1),
                        rhs.get(col, 2),
                        rhs.get(col, 3),
                    );
                    let rescol = lhs * rhscol;
                    self.set(col, 0, rescol[0]);
                    self.set(col, 1, rescol[1]);
                    self.set(col, 2, rescol[2]);
                    self.set(col, 3, rescol[3]);
                }
            }
        }

        // matrix * matrix
        impl std::ops::Mul<$t> for $t {
            type Output = $t;
            #[inline]
            fn mul(mut self, rhs: $t) -> $t {
                self *= rhs;
                self
            }
        }

        // matrix * vector
        impl std::ops::Mul<Vec4<$it>> for $t {
            type Output = Vec4<$it>;
            #[inline]
            fn mul(self, v: Vec4<$it>) -> Vec4<$it> {
                let mut result = Vec4::default();
                for col in 0..$t::COLUMNS {
                    let lhscol = Vec4::new(
                        self.get(col, 0),
                        self.get(col, 1),
                        self.get(col, 2),
                        self.get(col, 3),
                    );
                    result += lhscol * v[col];
                }
                result
            }
        }
    };
}

impl_mat!(Mat4f, filament_math_mat4f, f32);
impl_mat!(Mat4, filament_math_mat4, f64);
impl_mat!(Mat3f, filament_math_mat3f, f32);
impl_mat!(Mat3, filament_math_mat3, f64);

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Quaternion<T: Copy> {
    pub vec: [T; 4],
}

impl<T: Copy> Quaternion<T> {
    const COMPONENTS: usize = 4;
    #[inline]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { vec: [x, y, z, w] }
    }
}

impl_named_vec!(Quaternion, f64, Quat, filament_math_quat);
impl_named_vec!(Quaternion, f32, Quatf, filament_math_quatf);
impl_named_vec!(Quaternion, half::f16, Quath, filament_math_quath);

#[cfg(test)]
mod tests {
    use super::{Double3, Mat4};

    #[test]
    fn rotation_test() {
        assert!(
            Mat4::rotation(360.0f64.to_radians(), Double3::new(0.0, 1.0, 0.0))
                .is_close_to(&Mat4::default())
        );
        assert!(
            Mat4::rotation(90.0f64.to_radians(), Double3::new(1.0, 0.0, 0.0)).is_close_to(&Mat4([
                1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ]))
        );
    }

    #[test]
    fn mat_mul() {
        assert_eq!(Mat4::default() * Mat4::default(), Mat4::default());

        let m1 = Mat4([
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
        ]);
        assert_eq!(m1 * Mat4::default(), m1);
        assert_eq!(Mat4::default() * m1, m1);
        assert_eq!(
            m1 * m1,
            Mat4([
                90.0, 100.0, 110.0, 120.0, 202.0, 228.0, 254.0, 280.0, 314.0, 356.0, 398.0, 440.0,
                426.0, 484.0, 542.0, 600.0
            ])
        );
    }
}
