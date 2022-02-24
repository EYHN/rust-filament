use std::{os::raw, ptr};

use bitflags::bitflags;

use crate::bindgen;

bitflags! {
    #[derive(Default)]
    pub struct SwapChainConfig: u64 {
        const TRANSPARENT = bindgen::filament_SwapChain_CONFIG_TRANSPARENT;
        const READABLE = bindgen::filament_SwapChain_CONFIG_READABLE;
        const ENABLE_XCB = bindgen::filament_SwapChain_CONFIG_ENABLE_XCB;
        const APPLE_CVPIXELBUFFER = bindgen::filament_SwapChain_CONFIG_APPLE_CVPIXELBUFFER;
    }
}

pub struct SwapChain {
    native: ptr::NonNull<bindgen::filament_SwapChain>,
}

impl SwapChain {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_SwapChain {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_SwapChain {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_SwapChain) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(SwapChain { native: ptr })
    }

    #[inline]
    pub fn get_native_window(&self) -> *mut raw::c_void {
        unsafe { bindgen::filament_SwapChain_getNativeWindow(self.native()) }
    }

    pub fn set_frame_scheduled_callback(&mut self, _: Option<fn()>) {
        todo!()
    }

    pub fn set_frame_completed_callback(&mut self, _: Option<fn()>) {
        todo!()
    }
}
