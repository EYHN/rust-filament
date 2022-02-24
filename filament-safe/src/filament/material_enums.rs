#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use filament_bindings::{
    filament_BlendingMode_ADD, filament_BlendingMode_FADE, filament_BlendingMode_MASKED,
    filament_BlendingMode_MULTIPLY, filament_BlendingMode_OPAQUE, filament_BlendingMode_SCREEN,
    filament_BlendingMode_TRANSPARENT, filament_Interpolation_FLAT, filament_Interpolation_SMOOTH,
    filament_MaterialDomain_POST_PROCESS, filament_MaterialDomain_SURFACE,
    filament_Property_ABSORPTION, filament_Property_AMBIENT_OCCLUSION,
    filament_Property_ANISOTROPY, filament_Property_ANISOTROPY_DIRECTION,
    filament_Property_BASE_COLOR, filament_Property_BENT_NORMAL, filament_Property_CLEAR_COAT,
    filament_Property_CLEAR_COAT_NORMAL, filament_Property_CLEAR_COAT_ROUGHNESS,
    filament_Property_CLIP_SPACE_TRANSFORM, filament_Property_EMISSIVE,
    filament_Property_GLOSSINESS, filament_Property_IOR, filament_Property_METALLIC,
    filament_Property_MICRO_THICKNESS, filament_Property_NORMAL,
    filament_Property_POST_LIGHTING_COLOR, filament_Property_REFLECTANCE,
    filament_Property_ROUGHNESS, filament_Property_SHEEN_COLOR, filament_Property_SHEEN_ROUGHNESS,
    filament_Property_SPECULAR_COLOR, filament_Property_SUBSURFACE_COLOR,
    filament_Property_SUBSURFACE_POWER, filament_Property_THICKNESS,
    filament_Property_TRANSMISSION, filament_ReflectionMode_DEFAULT,
    filament_ReflectionMode_SCREEN_SPACE, filament_RefractionMode_CUBEMAP,
    filament_RefractionMode_NONE, filament_RefractionMode_SCREEN_SPACE,
    filament_RefractionType_SOLID, filament_RefractionType_THIN, filament_ShaderQuality_DEFAULT,
    filament_ShaderQuality_HIGH, filament_ShaderQuality_LOW, filament_ShaderQuality_NORMAL,
    filament_Shading_CLOTH, filament_Shading_LIT, filament_Shading_SPECULAR_GLOSSINESS,
    filament_Shading_SUBSURFACE, filament_Shading_UNLIT,
    filament_SpecularAmbientOcclusion_BENT_NORMALS, filament_SpecularAmbientOcclusion_NONE,
    filament_SpecularAmbientOcclusion_SIMPLE, filament_TransparencyMode_DEFAULT,
    filament_TransparencyMode_TWO_PASSES_ONE_SIDE, filament_TransparencyMode_TWO_PASSES_TWO_SIDES,
    filament_VertexAttribute_BONE_INDICES, filament_VertexAttribute_BONE_WEIGHTS,
    filament_VertexAttribute_COLOR, filament_VertexAttribute_CUSTOM0,
    filament_VertexAttribute_CUSTOM1, filament_VertexAttribute_CUSTOM2,
    filament_VertexAttribute_CUSTOM3, filament_VertexAttribute_CUSTOM4,
    filament_VertexAttribute_CUSTOM5, filament_VertexAttribute_CUSTOM6,
    filament_VertexAttribute_CUSTOM7, filament_VertexAttribute_MORPH_POSITION_0,
    filament_VertexAttribute_MORPH_POSITION_1, filament_VertexAttribute_MORPH_POSITION_2,
    filament_VertexAttribute_MORPH_POSITION_3, filament_VertexAttribute_MORPH_TANGENTS_0,
    filament_VertexAttribute_MORPH_TANGENTS_1, filament_VertexAttribute_MORPH_TANGENTS_2,
    filament_VertexAttribute_MORPH_TANGENTS_3, filament_VertexAttribute_POSITION,
    filament_VertexAttribute_TANGENTS, filament_VertexAttribute_UV0, filament_VertexAttribute_UV1,
    filament_VertexDomain_DEVICE, filament_VertexDomain_OBJECT, filament_VertexDomain_VIEW,
    filament_VertexDomain_WORLD,
};
use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum VertexAttribute {
    POSITION = filament_VertexAttribute_POSITION,
    TANGENTS = filament_VertexAttribute_TANGENTS,
    COLOR = filament_VertexAttribute_COLOR,
    UV0 = filament_VertexAttribute_UV0,
    UV1 = filament_VertexAttribute_UV1,
    BONE_INDICES = filament_VertexAttribute_BONE_INDICES,
    BONE_WEIGHTS = filament_VertexAttribute_BONE_WEIGHTS,
    CUSTOM0 = filament_VertexAttribute_CUSTOM0,
    CUSTOM1 = filament_VertexAttribute_CUSTOM1,
    CUSTOM2 = filament_VertexAttribute_CUSTOM2,
    CUSTOM3 = filament_VertexAttribute_CUSTOM3,
    CUSTOM4 = filament_VertexAttribute_CUSTOM4,
    CUSTOM5 = filament_VertexAttribute_CUSTOM5,
    CUSTOM6 = filament_VertexAttribute_CUSTOM6,
    CUSTOM7 = filament_VertexAttribute_CUSTOM7,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

impl VertexAttribute {
    pub fn MORPH_POSITION_0() -> Self {
        Self::from(filament_VertexAttribute_MORPH_POSITION_0)
    }
    pub fn MORPH_POSITION_1() -> Self {
        Self::from(filament_VertexAttribute_MORPH_POSITION_1)
    }
    pub fn MORPH_POSITION_2() -> Self {
        Self::from(filament_VertexAttribute_MORPH_POSITION_2)
    }
    pub fn MORPH_POSITION_3() -> Self {
        Self::from(filament_VertexAttribute_MORPH_POSITION_3)
    }
    pub fn MORPH_TANGENTS_0() -> Self {
        Self::from(filament_VertexAttribute_MORPH_TANGENTS_0)
    }
    pub fn MORPH_TANGENTS_1() -> Self {
        Self::from(filament_VertexAttribute_MORPH_TANGENTS_1)
    }
    pub fn MORPH_TANGENTS_2() -> Self {
        Self::from(filament_VertexAttribute_MORPH_TANGENTS_2)
    }
    pub fn MORPH_TANGENTS_3() -> Self {
        Self::from(filament_VertexAttribute_MORPH_TANGENTS_3)
    }
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum Shading {
    UNLIT = filament_Shading_UNLIT,
    LIT = filament_Shading_LIT,
    SUBSURFACE = filament_Shading_SUBSURFACE,
    CLOTH = filament_Shading_CLOTH,
    SPECULAR_GLOSSINESS = filament_Shading_SPECULAR_GLOSSINESS,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum Interpolation {
    SMOOTH = filament_Interpolation_SMOOTH,
    FLAT = filament_Interpolation_FLAT,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(i8)]
pub enum ShaderQuality {
    DEFAULT = filament_ShaderQuality_DEFAULT,
    LOW = filament_ShaderQuality_LOW,
    NORMAL = filament_ShaderQuality_NORMAL,
    HIGH = filament_ShaderQuality_HIGH,
    #[num_enum(default)]
    UNKNOWN = i8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum BlendingMode {
    OPAQUE = filament_BlendingMode_OPAQUE,
    TRANSPARENT = filament_BlendingMode_TRANSPARENT,
    ADD = filament_BlendingMode_ADD,
    MASKED = filament_BlendingMode_MASKED,
    FADE = filament_BlendingMode_FADE,
    MULTIPLY = filament_BlendingMode_MULTIPLY,
    SCREEN = filament_BlendingMode_SCREEN,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum TransparencyMode {
    DEFAULT = filament_TransparencyMode_DEFAULT,
    TWO_PASSES_ONE_SIDE = filament_TransparencyMode_TWO_PASSES_ONE_SIDE,
    TWO_PASSES_TWO_SIDES = filament_TransparencyMode_TWO_PASSES_TWO_SIDES,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum VertexDomain {
    OBJECT = filament_VertexDomain_OBJECT,
    WORLD = filament_VertexDomain_WORLD,
    VIEW = filament_VertexDomain_VIEW,
    DEVICE = filament_VertexDomain_DEVICE,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum MaterialDomain {
    SURFACE = filament_MaterialDomain_SURFACE,
    POST_PROCESS = filament_MaterialDomain_POST_PROCESS,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SpecularAmbientOcclusion {
    NONE = filament_SpecularAmbientOcclusion_NONE,
    SIMPLE = filament_SpecularAmbientOcclusion_SIMPLE,
    BENT_NORMALS = filament_SpecularAmbientOcclusion_BENT_NORMALS,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum RefractionMode {
    NONE = filament_RefractionMode_NONE,
    CUBEMAP = filament_RefractionMode_CUBEMAP,
    SCREEN_SPACE = filament_RefractionMode_SCREEN_SPACE,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum RefractionType {
    SOLID = filament_RefractionType_SOLID,
    THIN = filament_RefractionType_THIN,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum ReflectionMode {
    DEFAULT = filament_ReflectionMode_DEFAULT,
    SCREEN_SPACE = filament_ReflectionMode_SCREEN_SPACE,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum Property {
    BASE_COLOR = filament_Property_BASE_COLOR,
    ROUGHNESS = filament_Property_ROUGHNESS,
    METALLIC = filament_Property_METALLIC,
    REFLECTANCE = filament_Property_REFLECTANCE,
    AMBIENT_OCCLUSION = filament_Property_AMBIENT_OCCLUSION,
    CLEAR_COAT = filament_Property_CLEAR_COAT,
    CLEAR_COAT_ROUGHNESS = filament_Property_CLEAR_COAT_ROUGHNESS,
    CLEAR_COAT_NORMAL = filament_Property_CLEAR_COAT_NORMAL,
    ANISOTROPY = filament_Property_ANISOTROPY,
    ANISOTROPY_DIRECTION = filament_Property_ANISOTROPY_DIRECTION,
    THICKNESS = filament_Property_THICKNESS,
    SUBSURFACE_POWER = filament_Property_SUBSURFACE_POWER,
    SUBSURFACE_COLOR = filament_Property_SUBSURFACE_COLOR,
    SHEEN_COLOR = filament_Property_SHEEN_COLOR,
    SHEEN_ROUGHNESS = filament_Property_SHEEN_ROUGHNESS,
    SPECULAR_COLOR = filament_Property_SPECULAR_COLOR,
    GLOSSINESS = filament_Property_GLOSSINESS,
    EMISSIVE = filament_Property_EMISSIVE,
    NORMAL = filament_Property_NORMAL,
    POST_LIGHTING_COLOR = filament_Property_POST_LIGHTING_COLOR,
    CLIP_SPACE_TRANSFORM = filament_Property_CLIP_SPACE_TRANSFORM,
    ABSORPTION = filament_Property_ABSORPTION,
    TRANSMISSION = filament_Property_TRANSMISSION,
    IOR = filament_Property_IOR,
    MICRO_THICKNESS = filament_Property_MICRO_THICKNESS,
    BENT_NORMAL = filament_Property_BENT_NORMAL,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}
