use num_enum::{IntoPrimitive, FromPrimitive};

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum Backend {
    DEFAULT = 0,
    OPENGL = 1,
    VULKAN = 2,
    METAL = 3,
    NOOP = 4,
    #[num_enum(default)]
    UNKNOWN = 255
}
