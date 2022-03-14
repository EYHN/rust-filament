macro_rules! math_vec {
    ($len:expr, $t:ident, $ctype:ty) => {
        #[repr(transparent)]
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $t {
            pub inner: [$ctype; $len],
        }
    };
}

math_vec!(4, filament_math_bool4, ::std::os::raw::c_char);
math_vec!(4, filament_math_int4, ::std::os::raw::c_int);
math_vec!(4, filament_math_uint4, ::std::os::raw::c_uint);
math_vec!(4, filament_math_double4, ::std::os::raw::c_double);
math_vec!(4, filament_math_float4, ::std::os::raw::c_float);
math_vec!(4, filament_math_short4, ::std::os::raw::c_short);
math_vec!(4, filament_math_ushort4, ::std::os::raw::c_ushort);
math_vec!(4, filament_math_half4, ::half::f16);

math_vec!(3, filament_math_bool3, ::std::os::raw::c_char);
math_vec!(3, filament_math_int3, ::std::os::raw::c_int);
math_vec!(3, filament_math_uint3, ::std::os::raw::c_uint);
math_vec!(3, filament_math_double3, ::std::os::raw::c_double);
math_vec!(3, filament_math_float3, ::std::os::raw::c_float);
math_vec!(3, filament_math_short3, ::std::os::raw::c_short);
math_vec!(3, filament_math_ushort3, ::std::os::raw::c_ushort);
math_vec!(3, filament_math_half3, ::half::f16);

math_vec!(2, filament_math_bool2, ::std::os::raw::c_char);
math_vec!(2, filament_math_int2, ::std::os::raw::c_int);
math_vec!(2, filament_math_uint2, ::std::os::raw::c_uint);
math_vec!(2, filament_math_double2, ::std::os::raw::c_double);
math_vec!(2, filament_math_float2, ::std::os::raw::c_float);
math_vec!(2, filament_math_short2, ::std::os::raw::c_short);
math_vec!(2, filament_math_ushort2, ::std::os::raw::c_ushort);
math_vec!(2, filament_math_half2, ::half::f16);

math_vec!(16, filament_math_mat4, ::std::os::raw::c_double);
math_vec!(16, filament_math_mat4f, ::std::os::raw::c_float);

math_vec!(9, filament_math_mat3, ::std::os::raw::c_double);
math_vec!(9, filament_math_mat3f, ::std::os::raw::c_float);

math_vec!(4, filament_math_mat2, ::std::os::raw::c_double);
math_vec!(4, filament_math_mat2f, ::std::os::raw::c_float);

math_vec!(4, filament_math_quat, ::std::os::raw::c_double);
math_vec!(4, filament_math_quatf, ::std::os::raw::c_float);
math_vec!(4, filament_math_quath, ::half::f16);
