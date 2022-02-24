#![allow(non_camel_case_types)]

use num_enum::{FromPrimitive, IntoPrimitive};

use bitflags::bitflags;

use crate::bindgen;

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum ElementType {
    BYTE = bindgen::filament_backend_ElementType_BYTE,
    BYTE2 = bindgen::filament_backend_ElementType_BYTE2,
    BYTE3 = bindgen::filament_backend_ElementType_BYTE3,
    BYTE4 = bindgen::filament_backend_ElementType_BYTE4,
    UBYTE = bindgen::filament_backend_ElementType_UBYTE,
    UBYTE2 = bindgen::filament_backend_ElementType_UBYTE2,
    UBYTE3 = bindgen::filament_backend_ElementType_UBYTE3,
    UBYTE4 = bindgen::filament_backend_ElementType_UBYTE4,
    SHORT = bindgen::filament_backend_ElementType_SHORT,
    SHORT2 = bindgen::filament_backend_ElementType_SHORT2,
    SHORT3 = bindgen::filament_backend_ElementType_SHORT3,
    SHORT4 = bindgen::filament_backend_ElementType_SHORT4,
    USHORT = bindgen::filament_backend_ElementType_USHORT,
    USHORT2 = bindgen::filament_backend_ElementType_USHORT2,
    USHORT3 = bindgen::filament_backend_ElementType_USHORT3,
    USHORT4 = bindgen::filament_backend_ElementType_USHORT4,
    INT = bindgen::filament_backend_ElementType_INT,
    UINT = bindgen::filament_backend_ElementType_UINT,
    FLOAT = bindgen::filament_backend_ElementType_FLOAT,
    FLOAT2 = bindgen::filament_backend_ElementType_FLOAT2,
    FLOAT3 = bindgen::filament_backend_ElementType_FLOAT3,
    FLOAT4 = bindgen::filament_backend_ElementType_FLOAT4,
    HALF = bindgen::filament_backend_ElementType_HALF,
    HALF2 = bindgen::filament_backend_ElementType_HALF2,
    HALF3 = bindgen::filament_backend_ElementType_HALF3,
    HALF4 = bindgen::filament_backend_ElementType_HALF4,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerType {
    SAMPLER_2D = bindgen::filament_backend_SamplerType_SAMPLER_2D,
    SAMPLER_2D_ARRAY = bindgen::filament_backend_SamplerType_SAMPLER_2D_ARRAY,
    SAMPLER_CUBEMAP = bindgen::filament_backend_SamplerType_SAMPLER_CUBEMAP,
    SAMPLER_EXTERNAL = bindgen::filament_backend_SamplerType_SAMPLER_EXTERNAL,
    SAMPLER_3D = bindgen::filament_backend_SamplerType_SAMPLER_3D,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u16)]
pub enum TextureFormat {
    R8 = bindgen::filament_backend_TextureFormat_R8,
    R8_SNORM = bindgen::filament_backend_TextureFormat_R8_SNORM,
    R8UI = bindgen::filament_backend_TextureFormat_R8UI,
    R8I = bindgen::filament_backend_TextureFormat_R8I,
    STENCIL8 = bindgen::filament_backend_TextureFormat_STENCIL8,
    R16F = bindgen::filament_backend_TextureFormat_R16F,
    R16UI = bindgen::filament_backend_TextureFormat_R16UI,
    R16I = bindgen::filament_backend_TextureFormat_R16I,
    RG8 = bindgen::filament_backend_TextureFormat_RG8,
    RG8_SNORM = bindgen::filament_backend_TextureFormat_RG8_SNORM,
    RG8UI = bindgen::filament_backend_TextureFormat_RG8UI,
    RG8I = bindgen::filament_backend_TextureFormat_RG8I,
    RGB565 = bindgen::filament_backend_TextureFormat_RGB565,
    RGB9_E5 = bindgen::filament_backend_TextureFormat_RGB9_E5,
    RGB5_A1 = bindgen::filament_backend_TextureFormat_RGB5_A1,
    RGBA4 = bindgen::filament_backend_TextureFormat_RGBA4,
    DEPTH16 = bindgen::filament_backend_TextureFormat_DEPTH16,
    RGB8 = bindgen::filament_backend_TextureFormat_RGB8,
    SRGB8 = bindgen::filament_backend_TextureFormat_SRGB8,
    RGB8_SNORM = bindgen::filament_backend_TextureFormat_RGB8_SNORM,
    RGB8UI = bindgen::filament_backend_TextureFormat_RGB8UI,
    RGB8I = bindgen::filament_backend_TextureFormat_RGB8I,
    DEPTH24 = bindgen::filament_backend_TextureFormat_DEPTH24,
    R32F = bindgen::filament_backend_TextureFormat_R32F,
    R32UI = bindgen::filament_backend_TextureFormat_R32UI,
    R32I = bindgen::filament_backend_TextureFormat_R32I,
    RG16F = bindgen::filament_backend_TextureFormat_RG16F,
    RG16UI = bindgen::filament_backend_TextureFormat_RG16UI,
    RG16I = bindgen::filament_backend_TextureFormat_RG16I,
    R11F_G11F_B10F = bindgen::filament_backend_TextureFormat_R11F_G11F_B10F,
    RGBA8 = bindgen::filament_backend_TextureFormat_RGBA8,
    SRGB8_A8 = bindgen::filament_backend_TextureFormat_SRGB8_A8,
    RGBA8_SNORM = bindgen::filament_backend_TextureFormat_RGBA8_SNORM,
    UNUSED = bindgen::filament_backend_TextureFormat_UNUSED,
    RGB10_A2 = bindgen::filament_backend_TextureFormat_RGB10_A2,
    RGBA8UI = bindgen::filament_backend_TextureFormat_RGBA8UI,
    RGBA8I = bindgen::filament_backend_TextureFormat_RGBA8I,
    DEPTH32F = bindgen::filament_backend_TextureFormat_DEPTH32F,
    DEPTH24_STENCIL8 = bindgen::filament_backend_TextureFormat_DEPTH24_STENCIL8,
    DEPTH32F_STENCIL8 = bindgen::filament_backend_TextureFormat_DEPTH32F_STENCIL8,
    RGB16F = bindgen::filament_backend_TextureFormat_RGB16F,
    RGB16UI = bindgen::filament_backend_TextureFormat_RGB16UI,
    RGB16I = bindgen::filament_backend_TextureFormat_RGB16I,
    RG32F = bindgen::filament_backend_TextureFormat_RG32F,
    RG32UI = bindgen::filament_backend_TextureFormat_RG32UI,
    RG32I = bindgen::filament_backend_TextureFormat_RG32I,
    RGBA16F = bindgen::filament_backend_TextureFormat_RGBA16F,
    RGBA16UI = bindgen::filament_backend_TextureFormat_RGBA16UI,
    RGBA16I = bindgen::filament_backend_TextureFormat_RGBA16I,
    RGB32F = bindgen::filament_backend_TextureFormat_RGB32F,
    RGB32UI = bindgen::filament_backend_TextureFormat_RGB32UI,
    RGB32I = bindgen::filament_backend_TextureFormat_RGB32I,
    RGBA32F = bindgen::filament_backend_TextureFormat_RGBA32F,
    RGBA32UI = bindgen::filament_backend_TextureFormat_RGBA32UI,
    RGBA32I = bindgen::filament_backend_TextureFormat_RGBA32I,
    EAC_R11 = bindgen::filament_backend_TextureFormat_EAC_R11,
    EAC_R11_SIGNED = bindgen::filament_backend_TextureFormat_EAC_R11_SIGNED,
    EAC_RG11 = bindgen::filament_backend_TextureFormat_EAC_RG11,
    EAC_RG11_SIGNED = bindgen::filament_backend_TextureFormat_EAC_RG11_SIGNED,
    ETC2_RGB8 = bindgen::filament_backend_TextureFormat_ETC2_RGB8,
    ETC2_SRGB8 = bindgen::filament_backend_TextureFormat_ETC2_SRGB8,
    ETC2_RGB8_A1 = bindgen::filament_backend_TextureFormat_ETC2_RGB8_A1,
    ETC2_SRGB8_A1 = bindgen::filament_backend_TextureFormat_ETC2_SRGB8_A1,
    ETC2_EAC_RGBA8 = bindgen::filament_backend_TextureFormat_ETC2_EAC_RGBA8,
    ETC2_EAC_SRGBA8 = bindgen::filament_backend_TextureFormat_ETC2_EAC_SRGBA8,
    DXT1_RGB = bindgen::filament_backend_TextureFormat_DXT1_RGB,
    DXT1_RGBA = bindgen::filament_backend_TextureFormat_DXT1_RGBA,
    DXT3_RGBA = bindgen::filament_backend_TextureFormat_DXT3_RGBA,
    DXT5_RGBA = bindgen::filament_backend_TextureFormat_DXT5_RGBA,
    DXT1_SRGB = bindgen::filament_backend_TextureFormat_DXT1_SRGB,
    DXT1_SRGBA = bindgen::filament_backend_TextureFormat_DXT1_SRGBA,
    DXT3_SRGBA = bindgen::filament_backend_TextureFormat_DXT3_SRGBA,
    DXT5_SRGBA = bindgen::filament_backend_TextureFormat_DXT5_SRGBA,
    RGBA_ASTC_4x4 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_4x4,
    RGBA_ASTC_5x4 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_5x4,
    RGBA_ASTC_5x5 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_5x5,
    RGBA_ASTC_6x5 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_6x5,
    RGBA_ASTC_6x6 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_6x6,
    RGBA_ASTC_8x5 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_8x5,
    RGBA_ASTC_8x6 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_8x6,
    RGBA_ASTC_8x8 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_8x8,
    RGBA_ASTC_10x5 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_10x5,
    RGBA_ASTC_10x6 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_10x6,
    RGBA_ASTC_10x8 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_10x8,
    RGBA_ASTC_10x10 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_10x10,
    RGBA_ASTC_12x10 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_12x10,
    RGBA_ASTC_12x12 = bindgen::filament_backend_TextureFormat_RGBA_ASTC_12x12,
    SRGB8_ALPHA8_ASTC_4x4 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_4x4,
    SRGB8_ALPHA8_ASTC_5x4 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_5x4,
    SRGB8_ALPHA8_ASTC_5x5 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_5x5,
    SRGB8_ALPHA8_ASTC_6x5 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_6x5,
    SRGB8_ALPHA8_ASTC_6x6 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_6x6,
    SRGB8_ALPHA8_ASTC_8x5 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x5,
    SRGB8_ALPHA8_ASTC_8x6 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x6,
    SRGB8_ALPHA8_ASTC_8x8 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x8,
    SRGB8_ALPHA8_ASTC_10x5 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x5,
    SRGB8_ALPHA8_ASTC_10x6 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x6,
    SRGB8_ALPHA8_ASTC_10x8 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x8,
    SRGB8_ALPHA8_ASTC_10x10 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x10,
    SRGB8_ALPHA8_ASTC_12x10 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_12x10,
    SRGB8_ALPHA8_ASTC_12x12 = bindgen::filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_12x12,
    #[num_enum(default)]
    UNKNOWN = u16::MAX,
}

bitflags! {
    #[derive(Default)]
    pub struct TextureUsage: bindgen::filament_backend_TextureUsage {
        const NONE = bindgen::filament_backend_TextureUsage_NONE;
        const COLOR_ATTACHMENT = bindgen::filament_backend_TextureUsage_COLOR_ATTACHMENT;
        const DEPTH_ATTACHMENT = bindgen::filament_backend_TextureUsage_DEPTH_ATTACHMENT;
        const STENCIL_ATTACHMENT = bindgen::filament_backend_TextureUsage_STENCIL_ATTACHMENT;
        const UPLOADABLE = bindgen::filament_backend_TextureUsage_UPLOADABLE;
        const SAMPLEABLE = bindgen::filament_backend_TextureUsage_SAMPLEABLE;
        const SUBPASS_INPUT = bindgen::filament_backend_TextureUsage_SUBPASS_INPUT;
        const DEFAULT = bindgen::filament_backend_TextureUsage_DEFAULT;
    }
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum TextureSwizzle {
    SUBSTITUTE_ZERO = bindgen::filament_backend_TextureSwizzle_SUBSTITUTE_ZERO,
    SUBSTITUTE_ONE = bindgen::filament_backend_TextureSwizzle_SUBSTITUTE_ONE,
    CHANNEL_0 = bindgen::filament_backend_TextureSwizzle_CHANNEL_0,
    CHANNEL_1 = bindgen::filament_backend_TextureSwizzle_CHANNEL_1,
    CHANNEL_2 = bindgen::filament_backend_TextureSwizzle_CHANNEL_2,
    CHANNEL_3 = bindgen::filament_backend_TextureSwizzle_CHANNEL_3,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum PixelDataFormat {
    R = bindgen::filament_backend_PixelDataFormat_R,
    R_INTEGER = bindgen::filament_backend_PixelDataFormat_R_INTEGER,
    RG = bindgen::filament_backend_PixelDataFormat_RG,
    RG_INTEGER = bindgen::filament_backend_PixelDataFormat_RG_INTEGER,
    RGB = bindgen::filament_backend_PixelDataFormat_RGB,
    RGB_INTEGER = bindgen::filament_backend_PixelDataFormat_RGB_INTEGER,
    RGBA = bindgen::filament_backend_PixelDataFormat_RGBA,
    RGBA_INTEGER = bindgen::filament_backend_PixelDataFormat_RGBA_INTEGER,
    UNUSED = bindgen::filament_backend_PixelDataFormat_UNUSED,
    DEPTH_COMPONENT = bindgen::filament_backend_PixelDataFormat_DEPTH_COMPONENT,
    DEPTH_STENCIL = bindgen::filament_backend_PixelDataFormat_DEPTH_STENCIL,
    ALPHA = bindgen::filament_backend_PixelDataFormat_ALPHA,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum PixelDataType {
    UBYTE = bindgen::filament_backend_PixelDataType_UBYTE,
    BYTE = bindgen::filament_backend_PixelDataType_BYTE,
    USHORT = bindgen::filament_backend_PixelDataType_USHORT,
    SHORT = bindgen::filament_backend_PixelDataType_SHORT,
    UINT = bindgen::filament_backend_PixelDataType_UINT,
    INT = bindgen::filament_backend_PixelDataType_INT,
    HALF = bindgen::filament_backend_PixelDataType_HALF,
    FLOAT = bindgen::filament_backend_PixelDataType_FLOAT,
    COMPRESSED = bindgen::filament_backend_PixelDataType_COMPRESSED,
    UINT_10F_11F_11F_REV = bindgen::filament_backend_PixelDataType_UINT_10F_11F_11F_REV,
    USHORT_565 = bindgen::filament_backend_PixelDataType_USHORT_565,
    UINT_2_10_10_10_REV = bindgen::filament_backend_PixelDataType_UINT_2_10_10_10_REV,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum CullingMode {
    NONE = bindgen::filament_backend_CullingMode_NONE,
    FRONT = bindgen::filament_backend_CullingMode_FRONT,
    BACK = bindgen::filament_backend_CullingMode_BACK,
    FRONT_AND_BACK = bindgen::filament_backend_CullingMode_FRONT_AND_BACK,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerWrapMode {
    CLAMP_TO_EDGE = bindgen::filament_backend_SamplerWrapMode_CLAMP_TO_EDGE,
    REPEAT = bindgen::filament_backend_SamplerWrapMode_REPEAT,
    MIRRORED_REPEAT = bindgen::filament_backend_SamplerWrapMode_MIRRORED_REPEAT,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerMinFilter {
    NEAREST = bindgen::filament_backend_SamplerMinFilter_NEAREST,
    LINEAR = bindgen::filament_backend_SamplerMinFilter_LINEAR,
    NEAREST_MIPMAP_NEAREST = bindgen::filament_backend_SamplerMinFilter_NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST = bindgen::filament_backend_SamplerMinFilter_LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR = bindgen::filament_backend_SamplerMinFilter_NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR = bindgen::filament_backend_SamplerMinFilter_LINEAR_MIPMAP_LINEAR,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerMagFilter {
    NEAREST = bindgen::filament_backend_SamplerMagFilter_NEAREST,
    LINEAR = bindgen::filament_backend_SamplerMagFilter_LINEAR,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerCompareMode {
    NONE = bindgen::filament_backend_SamplerCompareMode_NONE,
    COMPARE_TO_TEXTURE = bindgen::filament_backend_SamplerCompareMode_COMPARE_TO_TEXTURE,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerCompareFunc {
    LE = bindgen::filament_backend_SamplerCompareFunc_LE,
    GE = bindgen::filament_backend_SamplerCompareFunc_GE,
    L = bindgen::filament_backend_SamplerCompareFunc_L,
    G = bindgen::filament_backend_SamplerCompareFunc_G,
    E = bindgen::filament_backend_SamplerCompareFunc_E,
    NE = bindgen::filament_backend_SamplerCompareFunc_NE,
    A = bindgen::filament_backend_SamplerCompareFunc_A,
    N = bindgen::filament_backend_SamplerCompareFunc_N,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum PrimitiveType {
    POINTS = bindgen::filament_backend_PrimitiveType_POINTS,
    LINES = bindgen::filament_backend_PrimitiveType_LINES,
    LINE_STRIP = bindgen::filament_backend_PrimitiveType_LINE_STRIP,
    TRIANGLES = bindgen::filament_backend_PrimitiveType_TRIANGLES,
    TRIANGLE_STRIP = bindgen::filament_backend_PrimitiveType_TRIANGLE_STRIP,
    #[num_enum(default)]
    NONE = bindgen::filament_backend_PrimitiveType_NONE,
}
