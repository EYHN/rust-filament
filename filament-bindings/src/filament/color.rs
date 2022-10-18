#![allow(non_camel_case_types)]

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{
    bindgen,
    math::{Float3, Float4},
};

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum RgbType {
    sRGB = bindgen::filament_RgbType_sRGB,
    LINEAR = bindgen::filament_RgbType_LINEAR,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum RgbaType {
    sRGB = bindgen::filament_RgbaType_sRGB,
    LINEAR = bindgen::filament_RgbaType_LINEAR,
    PREMULTIPLIED_sRGB = bindgen::filament_RgbaType_PREMULTIPLIED_sRGB,
    PREMULTIPLIED_LINEAR = bindgen::filament_RgbaType_PREMULTIPLIED_LINEAR,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct sRGBColor(pub Float3);

impl sRGBColor {
    #[allow(dead_code)]
    pub(crate) fn native_ptr(&self) -> *const bindgen::filament_math_float3 {
        self.0.native_ptr()
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct LinearColor(pub Float3);

impl LinearColor {
    #[allow(dead_code)]
    pub(crate) fn native_ptr(&self) -> *const bindgen::filament_math_float3 {
        self.0.native_ptr()
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct LinearColorA(pub Float4);

impl LinearColorA {
    #[allow(dead_code)]
    pub(crate) fn native_ptr(&self) -> *const bindgen::filament_math_float4 {
        self.0.native_ptr()
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct sRGBColorA(pub Float4);

impl sRGBColorA {
    #[allow(dead_code)]
    pub(crate) fn native_ptr(&self) -> *const bindgen::filament_math_float4 {
        self.0.native_ptr()
    }
}

impl sRGBColor {
    pub unsafe fn to_linear_fast(&self) -> LinearColor {
        let mut f3 = Float3::default();
        bindgen::helper_color_toLinear_fast_sRGB(self.0.native_ptr(), f3.native_ptr_mut());
        LinearColor(f3)
    }
}
