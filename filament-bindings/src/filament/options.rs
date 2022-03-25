use num_enum::{FromPrimitive, IntoPrimitive};

use crate::bindgen;

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum BlendMode {
    OPAQUE = bindgen::filament_BlendMode_OPAQUE,
    TRANSLUCENT = bindgen::filament_BlendMode_TRANSLUCENT,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum QualityLevel {
    LOW = bindgen::filament_QualityLevel_LOW,
    MEDIUM = bindgen::filament_QualityLevel_MEDIUM,
    HIGH = bindgen::filament_QualityLevel_HIGH,
    ULTRA = bindgen::filament_QualityLevel_ULTRA,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum AntiAliasing {
    NONE = bindgen::filament_AntiAliasing_NONE,
    FXAA = bindgen::filament_AntiAliasing_FXAA,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[derive(Clone, PartialEq, Debug)]
#[repr(C)]
pub struct TemporalAntiAliasingOptions {
    filter_width: f32,
    feedback: f32,
    enabled: bool,
}

impl Default for TemporalAntiAliasingOptions {
    fn default() -> Self {
        Self {
            filter_width: 1.0,
            feedback: 0.4,
            enabled: false,
        }
    }
}

#[test]
fn temporal_anti_aliasing_options_test() {
    assert_eq!(
        std::mem::size_of::<TemporalAntiAliasingOptions>(),
        std::mem::size_of::<bindgen::filament_TemporalAntiAliasingOptions>(),
    );
}
