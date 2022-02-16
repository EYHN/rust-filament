#[repr(u8)]
pub enum Backend {
    DEFAULT = 0,
    OPENGL = 1,
    VULKAN = 2,
    METAL = 3,
    NOOP = 4
}

impl Into<u8> for Backend {
    fn into(self) -> u8 {
        self as u8
    }
}