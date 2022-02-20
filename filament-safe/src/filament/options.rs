use filament_bindings::{
    filament_BlendMode_OPAQUE, filament_BlendMode_TRANSLUCENT, filament_QualityLevel_HIGH,
    filament_QualityLevel_LOW, filament_QualityLevel_MEDIUM, filament_QualityLevel_ULTRA, filament_AntiAliasing_NONE, filament_AntiAliasing_FXAA,
};
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum BlendMode {
    OPAQUE = filament_BlendMode_OPAQUE,
    TRANSLUCENT = filament_BlendMode_TRANSLUCENT,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum QualityLevel {
    LOW = filament_QualityLevel_LOW,
    MEDIUM = filament_QualityLevel_MEDIUM,
    HIGH = filament_QualityLevel_HIGH,
    ULTRA = filament_QualityLevel_ULTRA,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum AntiAliasing {
    NONE = filament_AntiAliasing_NONE,
    FXAA = filament_AntiAliasing_FXAA,
    #[num_enum(default)]
    UNKNOWN = 255,
}
