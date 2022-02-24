#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::bindgen;

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum VertexAttribute {
    POSITION = bindgen::filament_VertexAttribute_POSITION,
    TANGENTS = bindgen::filament_VertexAttribute_TANGENTS,
    COLOR = bindgen::filament_VertexAttribute_COLOR,
    UV0 = bindgen::filament_VertexAttribute_UV0,
    UV1 = bindgen::filament_VertexAttribute_UV1,
    BONE_INDICES = bindgen::filament_VertexAttribute_BONE_INDICES,
    BONE_WEIGHTS = bindgen::filament_VertexAttribute_BONE_WEIGHTS,
    CUSTOM0 = bindgen::filament_VertexAttribute_CUSTOM0,
    CUSTOM1 = bindgen::filament_VertexAttribute_CUSTOM1,
    CUSTOM2 = bindgen::filament_VertexAttribute_CUSTOM2,
    CUSTOM3 = bindgen::filament_VertexAttribute_CUSTOM3,
    CUSTOM4 = bindgen::filament_VertexAttribute_CUSTOM4,
    CUSTOM5 = bindgen::filament_VertexAttribute_CUSTOM5,
    CUSTOM6 = bindgen::filament_VertexAttribute_CUSTOM6,
    CUSTOM7 = bindgen::filament_VertexAttribute_CUSTOM7,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

impl VertexAttribute {
    pub fn MORPH_POSITION_0() -> Self {
        Self::from(bindgen::filament_VertexAttribute_MORPH_POSITION_0)
    }
    pub fn MORPH_POSITION_1() -> Self {
        Self::from(bindgen::filament_VertexAttribute_MORPH_POSITION_1)
    }
    pub fn MORPH_POSITION_2() -> Self {
        Self::from(bindgen::filament_VertexAttribute_MORPH_POSITION_2)
    }
    pub fn MORPH_POSITION_3() -> Self {
        Self::from(bindgen::filament_VertexAttribute_MORPH_POSITION_3)
    }
    pub fn MORPH_TANGENTS_0() -> Self {
        Self::from(bindgen::filament_VertexAttribute_MORPH_TANGENTS_0)
    }
    pub fn MORPH_TANGENTS_1() -> Self {
        Self::from(bindgen::filament_VertexAttribute_MORPH_TANGENTS_1)
    }
    pub fn MORPH_TANGENTS_2() -> Self {
        Self::from(bindgen::filament_VertexAttribute_MORPH_TANGENTS_2)
    }
    pub fn MORPH_TANGENTS_3() -> Self {
        Self::from(bindgen::filament_VertexAttribute_MORPH_TANGENTS_3)
    }
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum Shading {
    UNLIT = bindgen::filament_Shading_UNLIT,
    LIT = bindgen::filament_Shading_LIT,
    SUBSURFACE = bindgen::filament_Shading_SUBSURFACE,
    CLOTH = bindgen::filament_Shading_CLOTH,
    SPECULAR_GLOSSINESS = bindgen::filament_Shading_SPECULAR_GLOSSINESS,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum Interpolation {
    SMOOTH = bindgen::filament_Interpolation_SMOOTH,
    FLAT = bindgen::filament_Interpolation_FLAT,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(i8)]
pub enum ShaderQuality {
    DEFAULT = bindgen::filament_ShaderQuality_DEFAULT,
    LOW = bindgen::filament_ShaderQuality_LOW,
    NORMAL = bindgen::filament_ShaderQuality_NORMAL,
    HIGH = bindgen::filament_ShaderQuality_HIGH,
    #[num_enum(default)]
    UNKNOWN = i8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum BlendingMode {
    OPAQUE = bindgen::filament_BlendingMode_OPAQUE,
    TRANSPARENT = bindgen::filament_BlendingMode_TRANSPARENT,
    ADD = bindgen::filament_BlendingMode_ADD,
    MASKED = bindgen::filament_BlendingMode_MASKED,
    FADE = bindgen::filament_BlendingMode_FADE,
    MULTIPLY = bindgen::filament_BlendingMode_MULTIPLY,
    SCREEN = bindgen::filament_BlendingMode_SCREEN,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum TransparencyMode {
    DEFAULT = bindgen::filament_TransparencyMode_DEFAULT,
    TWO_PASSES_ONE_SIDE = bindgen::filament_TransparencyMode_TWO_PASSES_ONE_SIDE,
    TWO_PASSES_TWO_SIDES = bindgen::filament_TransparencyMode_TWO_PASSES_TWO_SIDES,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum VertexDomain {
    OBJECT = bindgen::filament_VertexDomain_OBJECT,
    WORLD = bindgen::filament_VertexDomain_WORLD,
    VIEW = bindgen::filament_VertexDomain_VIEW,
    DEVICE = bindgen::filament_VertexDomain_DEVICE,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum MaterialDomain {
    SURFACE = bindgen::filament_MaterialDomain_SURFACE,
    POST_PROCESS = bindgen::filament_MaterialDomain_POST_PROCESS,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SpecularAmbientOcclusion {
    NONE = bindgen::filament_SpecularAmbientOcclusion_NONE,
    SIMPLE = bindgen::filament_SpecularAmbientOcclusion_SIMPLE,
    BENT_NORMALS = bindgen::filament_SpecularAmbientOcclusion_BENT_NORMALS,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum RefractionMode {
    NONE = bindgen::filament_RefractionMode_NONE,
    CUBEMAP = bindgen::filament_RefractionMode_CUBEMAP,
    SCREEN_SPACE = bindgen::filament_RefractionMode_SCREEN_SPACE,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum RefractionType {
    SOLID = bindgen::filament_RefractionType_SOLID,
    THIN = bindgen::filament_RefractionType_THIN,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum ReflectionMode {
    DEFAULT = bindgen::filament_ReflectionMode_DEFAULT,
    SCREEN_SPACE = bindgen::filament_ReflectionMode_SCREEN_SPACE,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum Property {
    BASE_COLOR = bindgen::filament_Property_BASE_COLOR,
    ROUGHNESS = bindgen::filament_Property_ROUGHNESS,
    METALLIC = bindgen::filament_Property_METALLIC,
    REFLECTANCE = bindgen::filament_Property_REFLECTANCE,
    AMBIENT_OCCLUSION = bindgen::filament_Property_AMBIENT_OCCLUSION,
    CLEAR_COAT = bindgen::filament_Property_CLEAR_COAT,
    CLEAR_COAT_ROUGHNESS = bindgen::filament_Property_CLEAR_COAT_ROUGHNESS,
    CLEAR_COAT_NORMAL = bindgen::filament_Property_CLEAR_COAT_NORMAL,
    ANISOTROPY = bindgen::filament_Property_ANISOTROPY,
    ANISOTROPY_DIRECTION = bindgen::filament_Property_ANISOTROPY_DIRECTION,
    THICKNESS = bindgen::filament_Property_THICKNESS,
    SUBSURFACE_POWER = bindgen::filament_Property_SUBSURFACE_POWER,
    SUBSURFACE_COLOR = bindgen::filament_Property_SUBSURFACE_COLOR,
    SHEEN_COLOR = bindgen::filament_Property_SHEEN_COLOR,
    SHEEN_ROUGHNESS = bindgen::filament_Property_SHEEN_ROUGHNESS,
    SPECULAR_COLOR = bindgen::filament_Property_SPECULAR_COLOR,
    GLOSSINESS = bindgen::filament_Property_GLOSSINESS,
    EMISSIVE = bindgen::filament_Property_EMISSIVE,
    NORMAL = bindgen::filament_Property_NORMAL,
    POST_LIGHTING_COLOR = bindgen::filament_Property_POST_LIGHTING_COLOR,
    CLIP_SPACE_TRANSFORM = bindgen::filament_Property_CLIP_SPACE_TRANSFORM,
    ABSORPTION = bindgen::filament_Property_ABSORPTION,
    TRANSMISSION = bindgen::filament_Property_TRANSMISSION,
    IOR = bindgen::filament_Property_IOR,
    MICRO_THICKNESS = bindgen::filament_Property_MICRO_THICKNESS,
    BENT_NORMAL = bindgen::filament_Property_BENT_NORMAL,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}
