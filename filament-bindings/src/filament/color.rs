use num_enum::{FromPrimitive, IntoPrimitive};

use crate::bindgen;

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
