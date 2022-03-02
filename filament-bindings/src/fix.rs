macro_rules! math_vec {
    ($len:expr, $t:ident, $ctype:ident) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $t {
            pub inner: [::std::os::raw::$ctype; $len],
        }
    };
}

math_vec!(4, filament_math_bool4, c_char);
math_vec!(4, filament_math_int4, c_int);
math_vec!(4, filament_math_uint4, c_uint);
math_vec!(4, filament_math_double4, c_double);
math_vec!(4, filament_math_float4, c_float);
math_vec!(4, filament_math_short4, c_short);
math_vec!(3, filament_math_bool3, c_char);
math_vec!(3, filament_math_int3, c_int);
math_vec!(3, filament_math_uint3, c_uint);
math_vec!(3, filament_math_double3, c_double);
math_vec!(3, filament_math_float3, c_float);
math_vec!(3, filament_math_short3, c_short);
math_vec!(2, filament_math_bool2, c_char);
math_vec!(2, filament_math_int2, c_int);
math_vec!(2, filament_math_uint2, c_uint);
math_vec!(2, filament_math_double2, c_double);
math_vec!(2, filament_math_float2, c_float);
math_vec!(2, filament_math_short2, c_short);

math_vec!(16, filament_math_mat4, c_double);
math_vec!(16, filament_math_mat4f, c_float);

math_vec!(9, filament_math_mat3, c_double);
math_vec!(9, filament_math_mat3f, c_float);

math_vec!(4, filament_math_mat2, c_double);
math_vec!(4, filament_math_mat2f, c_float);

math_vec!(4, filament_math_quat, c_double);
math_vec!(4, filament_math_quatf, c_float);

#[repr(C)]
#[derive(Clone, Debug)]
pub struct utils_Slice<T, SizeType> {
    pub mBegin: *mut T,
    pub mEndOffset: SizeType,
}