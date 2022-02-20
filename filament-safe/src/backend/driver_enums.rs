#![allow(non_camel_case_types)]

use filament_bindings::{
    filament_backend_CullingMode_BACK, filament_backend_CullingMode_FRONT,
    filament_backend_CullingMode_FRONT_AND_BACK, filament_backend_CullingMode_NONE,
    filament_backend_ElementType_BYTE, filament_backend_ElementType_BYTE2,
    filament_backend_ElementType_BYTE3, filament_backend_ElementType_BYTE4,
    filament_backend_ElementType_FLOAT, filament_backend_ElementType_FLOAT2,
    filament_backend_ElementType_FLOAT3, filament_backend_ElementType_FLOAT4,
    filament_backend_ElementType_HALF, filament_backend_ElementType_HALF2,
    filament_backend_ElementType_HALF3, filament_backend_ElementType_HALF4,
    filament_backend_ElementType_INT, filament_backend_ElementType_SHORT,
    filament_backend_ElementType_SHORT2, filament_backend_ElementType_SHORT3,
    filament_backend_ElementType_SHORT4, filament_backend_ElementType_UBYTE,
    filament_backend_ElementType_UBYTE2, filament_backend_ElementType_UBYTE3,
    filament_backend_ElementType_UBYTE4, filament_backend_ElementType_UINT,
    filament_backend_ElementType_USHORT, filament_backend_ElementType_USHORT2,
    filament_backend_ElementType_USHORT3, filament_backend_ElementType_USHORT4,
    filament_backend_PixelDataFormat_ALPHA, filament_backend_PixelDataFormat_DEPTH_COMPONENT,
    filament_backend_PixelDataFormat_DEPTH_STENCIL, filament_backend_PixelDataFormat_R,
    filament_backend_PixelDataFormat_RG, filament_backend_PixelDataFormat_RGB,
    filament_backend_PixelDataFormat_RGBA, filament_backend_PixelDataFormat_RGBA_INTEGER,
    filament_backend_PixelDataFormat_RGB_INTEGER, filament_backend_PixelDataFormat_RG_INTEGER,
    filament_backend_PixelDataFormat_R_INTEGER, filament_backend_PixelDataFormat_UNUSED,
    filament_backend_PixelDataType_BYTE, filament_backend_PixelDataType_COMPRESSED,
    filament_backend_PixelDataType_FLOAT, filament_backend_PixelDataType_HALF,
    filament_backend_PixelDataType_INT, filament_backend_PixelDataType_SHORT,
    filament_backend_PixelDataType_UBYTE, filament_backend_PixelDataType_UINT,
    filament_backend_PixelDataType_UINT_10F_11F_11F_REV,
    filament_backend_PixelDataType_UINT_2_10_10_10_REV, filament_backend_PixelDataType_USHORT,
    filament_backend_PixelDataType_USHORT_565, filament_backend_SamplerCompareFunc_A,
    filament_backend_SamplerCompareFunc_E, filament_backend_SamplerCompareFunc_G,
    filament_backend_SamplerCompareFunc_GE, filament_backend_SamplerCompareFunc_L,
    filament_backend_SamplerCompareFunc_LE, filament_backend_SamplerCompareFunc_N,
    filament_backend_SamplerCompareFunc_NE, filament_backend_SamplerCompareMode_COMPARE_TO_TEXTURE,
    filament_backend_SamplerCompareMode_NONE, filament_backend_SamplerMagFilter_LINEAR,
    filament_backend_SamplerMagFilter_NEAREST, filament_backend_SamplerMinFilter_LINEAR,
    filament_backend_SamplerMinFilter_LINEAR_MIPMAP_LINEAR,
    filament_backend_SamplerMinFilter_LINEAR_MIPMAP_NEAREST,
    filament_backend_SamplerMinFilter_NEAREST,
    filament_backend_SamplerMinFilter_NEAREST_MIPMAP_LINEAR,
    filament_backend_SamplerMinFilter_NEAREST_MIPMAP_NEAREST,
    filament_backend_SamplerType_SAMPLER_2D, filament_backend_SamplerType_SAMPLER_2D_ARRAY,
    filament_backend_SamplerType_SAMPLER_3D, filament_backend_SamplerType_SAMPLER_CUBEMAP,
    filament_backend_SamplerType_SAMPLER_EXTERNAL, filament_backend_SamplerWrapMode_CLAMP_TO_EDGE,
    filament_backend_SamplerWrapMode_MIRRORED_REPEAT, filament_backend_SamplerWrapMode_REPEAT,
    filament_backend_TextureFormat_DEPTH16, filament_backend_TextureFormat_DEPTH24,
    filament_backend_TextureFormat_DEPTH24_STENCIL8, filament_backend_TextureFormat_DEPTH32F,
    filament_backend_TextureFormat_DEPTH32F_STENCIL8, filament_backend_TextureFormat_DXT1_RGB,
    filament_backend_TextureFormat_DXT1_RGBA, filament_backend_TextureFormat_DXT1_SRGB,
    filament_backend_TextureFormat_DXT1_SRGBA, filament_backend_TextureFormat_DXT3_RGBA,
    filament_backend_TextureFormat_DXT3_SRGBA, filament_backend_TextureFormat_DXT5_RGBA,
    filament_backend_TextureFormat_DXT5_SRGBA, filament_backend_TextureFormat_EAC_R11,
    filament_backend_TextureFormat_EAC_R11_SIGNED, filament_backend_TextureFormat_EAC_RG11,
    filament_backend_TextureFormat_EAC_RG11_SIGNED, filament_backend_TextureFormat_ETC2_EAC_RGBA8,
    filament_backend_TextureFormat_ETC2_EAC_SRGBA8, filament_backend_TextureFormat_ETC2_RGB8,
    filament_backend_TextureFormat_ETC2_RGB8_A1, filament_backend_TextureFormat_ETC2_SRGB8,
    filament_backend_TextureFormat_ETC2_SRGB8_A1, filament_backend_TextureFormat_R11F_G11F_B10F,
    filament_backend_TextureFormat_R16F, filament_backend_TextureFormat_R16I,
    filament_backend_TextureFormat_R16UI, filament_backend_TextureFormat_R32F,
    filament_backend_TextureFormat_R32I, filament_backend_TextureFormat_R32UI,
    filament_backend_TextureFormat_R8, filament_backend_TextureFormat_R8I,
    filament_backend_TextureFormat_R8UI, filament_backend_TextureFormat_R8_SNORM,
    filament_backend_TextureFormat_RG16F, filament_backend_TextureFormat_RG16I,
    filament_backend_TextureFormat_RG16UI, filament_backend_TextureFormat_RG32F,
    filament_backend_TextureFormat_RG32I, filament_backend_TextureFormat_RG32UI,
    filament_backend_TextureFormat_RG8, filament_backend_TextureFormat_RG8I,
    filament_backend_TextureFormat_RG8UI, filament_backend_TextureFormat_RG8_SNORM,
    filament_backend_TextureFormat_RGB10_A2, filament_backend_TextureFormat_RGB16F,
    filament_backend_TextureFormat_RGB16I, filament_backend_TextureFormat_RGB16UI,
    filament_backend_TextureFormat_RGB32F, filament_backend_TextureFormat_RGB32I,
    filament_backend_TextureFormat_RGB32UI, filament_backend_TextureFormat_RGB565,
    filament_backend_TextureFormat_RGB5_A1, filament_backend_TextureFormat_RGB8,
    filament_backend_TextureFormat_RGB8I, filament_backend_TextureFormat_RGB8UI,
    filament_backend_TextureFormat_RGB8_SNORM, filament_backend_TextureFormat_RGB9_E5,
    filament_backend_TextureFormat_RGBA16F, filament_backend_TextureFormat_RGBA16I,
    filament_backend_TextureFormat_RGBA16UI, filament_backend_TextureFormat_RGBA32F,
    filament_backend_TextureFormat_RGBA32I, filament_backend_TextureFormat_RGBA32UI,
    filament_backend_TextureFormat_RGBA4, filament_backend_TextureFormat_RGBA8,
    filament_backend_TextureFormat_RGBA8I, filament_backend_TextureFormat_RGBA8UI,
    filament_backend_TextureFormat_RGBA8_SNORM, filament_backend_TextureFormat_RGBA_ASTC_10x10,
    filament_backend_TextureFormat_RGBA_ASTC_10x5, filament_backend_TextureFormat_RGBA_ASTC_10x6,
    filament_backend_TextureFormat_RGBA_ASTC_10x8, filament_backend_TextureFormat_RGBA_ASTC_12x10,
    filament_backend_TextureFormat_RGBA_ASTC_12x12, filament_backend_TextureFormat_RGBA_ASTC_4x4,
    filament_backend_TextureFormat_RGBA_ASTC_5x4, filament_backend_TextureFormat_RGBA_ASTC_5x5,
    filament_backend_TextureFormat_RGBA_ASTC_6x5, filament_backend_TextureFormat_RGBA_ASTC_6x6,
    filament_backend_TextureFormat_RGBA_ASTC_8x5, filament_backend_TextureFormat_RGBA_ASTC_8x6,
    filament_backend_TextureFormat_RGBA_ASTC_8x8, filament_backend_TextureFormat_SRGB8,
    filament_backend_TextureFormat_SRGB8_A8,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x10,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x5,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x6,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x8,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_12x10,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_12x12,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_4x4,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_5x4,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_5x5,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_6x5,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_6x6,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x5,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x6,
    filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x8, filament_backend_TextureFormat_STENCIL8,
    filament_backend_TextureFormat_UNUSED, filament_backend_TextureSwizzle_CHANNEL_0,
    filament_backend_TextureSwizzle_CHANNEL_1, filament_backend_TextureSwizzle_CHANNEL_2,
    filament_backend_TextureSwizzle_CHANNEL_3, filament_backend_TextureSwizzle_SUBSTITUTE_ONE,
    filament_backend_TextureSwizzle_SUBSTITUTE_ZERO, filament_backend_TextureUsage,
    filament_backend_TextureUsage_COLOR_ATTACHMENT, filament_backend_TextureUsage_DEFAULT,
    filament_backend_TextureUsage_DEPTH_ATTACHMENT, filament_backend_TextureUsage_NONE,
    filament_backend_TextureUsage_SAMPLEABLE, filament_backend_TextureUsage_STENCIL_ATTACHMENT,
    filament_backend_TextureUsage_SUBPASS_INPUT, filament_backend_TextureUsage_UPLOADABLE,
};
use num_enum::{FromPrimitive, IntoPrimitive};

use bitflags::bitflags;

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum ElementType {
    BYTE = filament_backend_ElementType_BYTE,
    BYTE2 = filament_backend_ElementType_BYTE2,
    BYTE3 = filament_backend_ElementType_BYTE3,
    BYTE4 = filament_backend_ElementType_BYTE4,
    UBYTE = filament_backend_ElementType_UBYTE,
    UBYTE2 = filament_backend_ElementType_UBYTE2,
    UBYTE3 = filament_backend_ElementType_UBYTE3,
    UBYTE4 = filament_backend_ElementType_UBYTE4,
    SHORT = filament_backend_ElementType_SHORT,
    SHORT2 = filament_backend_ElementType_SHORT2,
    SHORT3 = filament_backend_ElementType_SHORT3,
    SHORT4 = filament_backend_ElementType_SHORT4,
    USHORT = filament_backend_ElementType_USHORT,
    USHORT2 = filament_backend_ElementType_USHORT2,
    USHORT3 = filament_backend_ElementType_USHORT3,
    USHORT4 = filament_backend_ElementType_USHORT4,
    INT = filament_backend_ElementType_INT,
    UINT = filament_backend_ElementType_UINT,
    FLOAT = filament_backend_ElementType_FLOAT,
    FLOAT2 = filament_backend_ElementType_FLOAT2,
    FLOAT3 = filament_backend_ElementType_FLOAT3,
    FLOAT4 = filament_backend_ElementType_FLOAT4,
    HALF = filament_backend_ElementType_HALF,
    HALF2 = filament_backend_ElementType_HALF2,
    HALF3 = filament_backend_ElementType_HALF3,
    HALF4 = filament_backend_ElementType_HALF4,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerType {
    SAMPLER_2D = filament_backend_SamplerType_SAMPLER_2D,
    SAMPLER_2D_ARRAY = filament_backend_SamplerType_SAMPLER_2D_ARRAY,
    SAMPLER_CUBEMAP = filament_backend_SamplerType_SAMPLER_CUBEMAP,
    SAMPLER_EXTERNAL = filament_backend_SamplerType_SAMPLER_EXTERNAL,
    SAMPLER_3D = filament_backend_SamplerType_SAMPLER_3D,
    #[num_enum(default)]
    UNKNOWN = 255,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u16)]
pub enum TextureFormat {
    R8 = filament_backend_TextureFormat_R8,
    R8_SNORM = filament_backend_TextureFormat_R8_SNORM,
    R8UI = filament_backend_TextureFormat_R8UI,
    R8I = filament_backend_TextureFormat_R8I,
    STENCIL8 = filament_backend_TextureFormat_STENCIL8,
    R16F = filament_backend_TextureFormat_R16F,
    R16UI = filament_backend_TextureFormat_R16UI,
    R16I = filament_backend_TextureFormat_R16I,
    RG8 = filament_backend_TextureFormat_RG8,
    RG8_SNORM = filament_backend_TextureFormat_RG8_SNORM,
    RG8UI = filament_backend_TextureFormat_RG8UI,
    RG8I = filament_backend_TextureFormat_RG8I,
    RGB565 = filament_backend_TextureFormat_RGB565,
    RGB9_E5 = filament_backend_TextureFormat_RGB9_E5,
    RGB5_A1 = filament_backend_TextureFormat_RGB5_A1,
    RGBA4 = filament_backend_TextureFormat_RGBA4,
    DEPTH16 = filament_backend_TextureFormat_DEPTH16,
    RGB8 = filament_backend_TextureFormat_RGB8,
    SRGB8 = filament_backend_TextureFormat_SRGB8,
    RGB8_SNORM = filament_backend_TextureFormat_RGB8_SNORM,
    RGB8UI = filament_backend_TextureFormat_RGB8UI,
    RGB8I = filament_backend_TextureFormat_RGB8I,
    DEPTH24 = filament_backend_TextureFormat_DEPTH24,
    R32F = filament_backend_TextureFormat_R32F,
    R32UI = filament_backend_TextureFormat_R32UI,
    R32I = filament_backend_TextureFormat_R32I,
    RG16F = filament_backend_TextureFormat_RG16F,
    RG16UI = filament_backend_TextureFormat_RG16UI,
    RG16I = filament_backend_TextureFormat_RG16I,
    R11F_G11F_B10F = filament_backend_TextureFormat_R11F_G11F_B10F,
    RGBA8 = filament_backend_TextureFormat_RGBA8,
    SRGB8_A8 = filament_backend_TextureFormat_SRGB8_A8,
    RGBA8_SNORM = filament_backend_TextureFormat_RGBA8_SNORM,
    UNUSED = filament_backend_TextureFormat_UNUSED,
    RGB10_A2 = filament_backend_TextureFormat_RGB10_A2,
    RGBA8UI = filament_backend_TextureFormat_RGBA8UI,
    RGBA8I = filament_backend_TextureFormat_RGBA8I,
    DEPTH32F = filament_backend_TextureFormat_DEPTH32F,
    DEPTH24_STENCIL8 = filament_backend_TextureFormat_DEPTH24_STENCIL8,
    DEPTH32F_STENCIL8 = filament_backend_TextureFormat_DEPTH32F_STENCIL8,
    RGB16F = filament_backend_TextureFormat_RGB16F,
    RGB16UI = filament_backend_TextureFormat_RGB16UI,
    RGB16I = filament_backend_TextureFormat_RGB16I,
    RG32F = filament_backend_TextureFormat_RG32F,
    RG32UI = filament_backend_TextureFormat_RG32UI,
    RG32I = filament_backend_TextureFormat_RG32I,
    RGBA16F = filament_backend_TextureFormat_RGBA16F,
    RGBA16UI = filament_backend_TextureFormat_RGBA16UI,
    RGBA16I = filament_backend_TextureFormat_RGBA16I,
    RGB32F = filament_backend_TextureFormat_RGB32F,
    RGB32UI = filament_backend_TextureFormat_RGB32UI,
    RGB32I = filament_backend_TextureFormat_RGB32I,
    RGBA32F = filament_backend_TextureFormat_RGBA32F,
    RGBA32UI = filament_backend_TextureFormat_RGBA32UI,
    RGBA32I = filament_backend_TextureFormat_RGBA32I,
    EAC_R11 = filament_backend_TextureFormat_EAC_R11,
    EAC_R11_SIGNED = filament_backend_TextureFormat_EAC_R11_SIGNED,
    EAC_RG11 = filament_backend_TextureFormat_EAC_RG11,
    EAC_RG11_SIGNED = filament_backend_TextureFormat_EAC_RG11_SIGNED,
    ETC2_RGB8 = filament_backend_TextureFormat_ETC2_RGB8,
    ETC2_SRGB8 = filament_backend_TextureFormat_ETC2_SRGB8,
    ETC2_RGB8_A1 = filament_backend_TextureFormat_ETC2_RGB8_A1,
    ETC2_SRGB8_A1 = filament_backend_TextureFormat_ETC2_SRGB8_A1,
    ETC2_EAC_RGBA8 = filament_backend_TextureFormat_ETC2_EAC_RGBA8,
    ETC2_EAC_SRGBA8 = filament_backend_TextureFormat_ETC2_EAC_SRGBA8,
    DXT1_RGB = filament_backend_TextureFormat_DXT1_RGB,
    DXT1_RGBA = filament_backend_TextureFormat_DXT1_RGBA,
    DXT3_RGBA = filament_backend_TextureFormat_DXT3_RGBA,
    DXT5_RGBA = filament_backend_TextureFormat_DXT5_RGBA,
    DXT1_SRGB = filament_backend_TextureFormat_DXT1_SRGB,
    DXT1_SRGBA = filament_backend_TextureFormat_DXT1_SRGBA,
    DXT3_SRGBA = filament_backend_TextureFormat_DXT3_SRGBA,
    DXT5_SRGBA = filament_backend_TextureFormat_DXT5_SRGBA,
    RGBA_ASTC_4x4 = filament_backend_TextureFormat_RGBA_ASTC_4x4,
    RGBA_ASTC_5x4 = filament_backend_TextureFormat_RGBA_ASTC_5x4,
    RGBA_ASTC_5x5 = filament_backend_TextureFormat_RGBA_ASTC_5x5,
    RGBA_ASTC_6x5 = filament_backend_TextureFormat_RGBA_ASTC_6x5,
    RGBA_ASTC_6x6 = filament_backend_TextureFormat_RGBA_ASTC_6x6,
    RGBA_ASTC_8x5 = filament_backend_TextureFormat_RGBA_ASTC_8x5,
    RGBA_ASTC_8x6 = filament_backend_TextureFormat_RGBA_ASTC_8x6,
    RGBA_ASTC_8x8 = filament_backend_TextureFormat_RGBA_ASTC_8x8,
    RGBA_ASTC_10x5 = filament_backend_TextureFormat_RGBA_ASTC_10x5,
    RGBA_ASTC_10x6 = filament_backend_TextureFormat_RGBA_ASTC_10x6,
    RGBA_ASTC_10x8 = filament_backend_TextureFormat_RGBA_ASTC_10x8,
    RGBA_ASTC_10x10 = filament_backend_TextureFormat_RGBA_ASTC_10x10,
    RGBA_ASTC_12x10 = filament_backend_TextureFormat_RGBA_ASTC_12x10,
    RGBA_ASTC_12x12 = filament_backend_TextureFormat_RGBA_ASTC_12x12,
    SRGB8_ALPHA8_ASTC_4x4 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_4x4,
    SRGB8_ALPHA8_ASTC_5x4 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_5x4,
    SRGB8_ALPHA8_ASTC_5x5 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_5x5,
    SRGB8_ALPHA8_ASTC_6x5 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_6x5,
    SRGB8_ALPHA8_ASTC_6x6 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_6x6,
    SRGB8_ALPHA8_ASTC_8x5 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x5,
    SRGB8_ALPHA8_ASTC_8x6 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x6,
    SRGB8_ALPHA8_ASTC_8x8 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_8x8,
    SRGB8_ALPHA8_ASTC_10x5 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x5,
    SRGB8_ALPHA8_ASTC_10x6 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x6,
    SRGB8_ALPHA8_ASTC_10x8 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x8,
    SRGB8_ALPHA8_ASTC_10x10 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_10x10,
    SRGB8_ALPHA8_ASTC_12x10 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_12x10,
    SRGB8_ALPHA8_ASTC_12x12 = filament_backend_TextureFormat_SRGB8_ALPHA8_ASTC_12x12,
    #[num_enum(default)]
    UNKNOWN = u16::MAX,
}

bitflags! {
    #[derive(Default)]
    pub struct TextureUsage: filament_backend_TextureUsage {
        const NONE = filament_backend_TextureUsage_NONE;
        const COLOR_ATTACHMENT = filament_backend_TextureUsage_COLOR_ATTACHMENT;
        const DEPTH_ATTACHMENT = filament_backend_TextureUsage_DEPTH_ATTACHMENT;
        const STENCIL_ATTACHMENT = filament_backend_TextureUsage_STENCIL_ATTACHMENT;
        const UPLOADABLE = filament_backend_TextureUsage_UPLOADABLE;
        const SAMPLEABLE = filament_backend_TextureUsage_SAMPLEABLE;
        const SUBPASS_INPUT = filament_backend_TextureUsage_SUBPASS_INPUT;
        const DEFAULT = filament_backend_TextureUsage_DEFAULT;
    }
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum TextureSwizzle {
    SUBSTITUTE_ZERO = filament_backend_TextureSwizzle_SUBSTITUTE_ZERO,
    SUBSTITUTE_ONE = filament_backend_TextureSwizzle_SUBSTITUTE_ONE,
    CHANNEL_0 = filament_backend_TextureSwizzle_CHANNEL_0,
    CHANNEL_1 = filament_backend_TextureSwizzle_CHANNEL_1,
    CHANNEL_2 = filament_backend_TextureSwizzle_CHANNEL_2,
    CHANNEL_3 = filament_backend_TextureSwizzle_CHANNEL_3,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum PixelDataFormat {
    R = filament_backend_PixelDataFormat_R,
    R_INTEGER = filament_backend_PixelDataFormat_R_INTEGER,
    RG = filament_backend_PixelDataFormat_RG,
    RG_INTEGER = filament_backend_PixelDataFormat_RG_INTEGER,
    RGB = filament_backend_PixelDataFormat_RGB,
    RGB_INTEGER = filament_backend_PixelDataFormat_RGB_INTEGER,
    RGBA = filament_backend_PixelDataFormat_RGBA,
    RGBA_INTEGER = filament_backend_PixelDataFormat_RGBA_INTEGER,
    UNUSED = filament_backend_PixelDataFormat_UNUSED,
    DEPTH_COMPONENT = filament_backend_PixelDataFormat_DEPTH_COMPONENT,
    DEPTH_STENCIL = filament_backend_PixelDataFormat_DEPTH_STENCIL,
    ALPHA = filament_backend_PixelDataFormat_ALPHA,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum PixelDataType {
    UBYTE = filament_backend_PixelDataType_UBYTE,
    BYTE = filament_backend_PixelDataType_BYTE,
    USHORT = filament_backend_PixelDataType_USHORT,
    SHORT = filament_backend_PixelDataType_SHORT,
    UINT = filament_backend_PixelDataType_UINT,
    INT = filament_backend_PixelDataType_INT,
    HALF = filament_backend_PixelDataType_HALF,
    FLOAT = filament_backend_PixelDataType_FLOAT,
    COMPRESSED = filament_backend_PixelDataType_COMPRESSED,
    UINT_10F_11F_11F_REV = filament_backend_PixelDataType_UINT_10F_11F_11F_REV,
    USHORT_565 = filament_backend_PixelDataType_USHORT_565,
    UINT_2_10_10_10_REV = filament_backend_PixelDataType_UINT_2_10_10_10_REV,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum CullingMode {
    NONE = filament_backend_CullingMode_NONE,
    FRONT = filament_backend_CullingMode_FRONT,
    BACK = filament_backend_CullingMode_BACK,
    FRONT_AND_BACK = filament_backend_CullingMode_FRONT_AND_BACK,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerWrapMode {
    CLAMP_TO_EDGE = filament_backend_SamplerWrapMode_CLAMP_TO_EDGE,
    REPEAT = filament_backend_SamplerWrapMode_REPEAT,
    MIRRORED_REPEAT = filament_backend_SamplerWrapMode_MIRRORED_REPEAT,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerMinFilter {
    NEAREST = filament_backend_SamplerMinFilter_NEAREST,
    LINEAR = filament_backend_SamplerMinFilter_LINEAR,
    NEAREST_MIPMAP_NEAREST = filament_backend_SamplerMinFilter_NEAREST_MIPMAP_NEAREST,
    LINEAR_MIPMAP_NEAREST = filament_backend_SamplerMinFilter_LINEAR_MIPMAP_NEAREST,
    NEAREST_MIPMAP_LINEAR = filament_backend_SamplerMinFilter_NEAREST_MIPMAP_LINEAR,
    LINEAR_MIPMAP_LINEAR = filament_backend_SamplerMinFilter_LINEAR_MIPMAP_LINEAR,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerMagFilter {
    NEAREST = filament_backend_SamplerMagFilter_NEAREST,
    LINEAR = filament_backend_SamplerMagFilter_LINEAR,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerCompareMode {
    NONE = filament_backend_SamplerCompareMode_NONE,
    COMPARE_TO_TEXTURE = filament_backend_SamplerCompareMode_COMPARE_TO_TEXTURE,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum SamplerCompareFunc {
    LE = filament_backend_SamplerCompareFunc_LE,
    GE = filament_backend_SamplerCompareFunc_GE,
    L = filament_backend_SamplerCompareFunc_L,
    G = filament_backend_SamplerCompareFunc_G,
    E = filament_backend_SamplerCompareFunc_E,
    NE = filament_backend_SamplerCompareFunc_NE,
    A = filament_backend_SamplerCompareFunc_A,
    N = filament_backend_SamplerCompareFunc_N,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}
