use std::os::raw;

use bitflags::bitflags;

use filament_bindings::{
    filament_SwapChain, filament_SwapChain_CONFIG_APPLE_CVPIXELBUFFER,
    filament_SwapChain_CONFIG_ENABLE_XCB, filament_SwapChain_CONFIG_READABLE,
    filament_SwapChain_CONFIG_TRANSPARENT, filament_SwapChain_getNativeWindow,
};

bitflags! {
    pub struct SwapChainConfig: u64 {
        const TRANSPARENT = filament_SwapChain_CONFIG_TRANSPARENT;
        const READABLE = filament_SwapChain_CONFIG_READABLE;
        const ENABLE_XCB = filament_SwapChain_CONFIG_ENABLE_XCB;
        const APPLE_CVPIXELBUFFER = filament_SwapChain_CONFIG_APPLE_CVPIXELBUFFER;
    }
}

pub struct SwapChain(*mut filament_SwapChain);

impl SwapChain {
    #[inline]
    pub fn from_ptr(raw_ptr: *mut filament_SwapChain) -> Option<Self> {
        if raw_ptr.is_null() {
            None
        } else {
            Some(Self(raw_ptr))
        }
    }

    #[inline]
    pub fn get_native_window(&self) -> *mut raw::c_void {
        unsafe { filament_SwapChain_getNativeWindow(self.0) }
    }

    pub fn set_frame_scheduled_callback(&mut self, callback: Option<fn()>) {
        todo!()
    }

    pub fn set_frame_completed_callback(&mut self, callback: Option<fn()>) {
        todo!()
    }
}
