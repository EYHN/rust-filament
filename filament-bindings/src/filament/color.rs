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
#[derive(Clone, Debug)]
struct sRGBColor(Float3);

#[repr(C)]
#[derive(Clone, Debug)]
struct LinearColor(Float3);

#[repr(C)]
#[derive(Clone, Debug)]
struct LinearColorA(Float4);

#[repr(C)]
#[derive(Clone, Debug)]
struct sRGBColorA(Float4);
