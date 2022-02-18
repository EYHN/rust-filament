use std::{os::raw, ptr, rc::Rc};

use bitflags::bitflags;

use filament_bindings::{
    filament_Engine_createSwapChain, filament_Engine_createSwapChain1, filament_Engine_destroy15,
    filament_SwapChain, filament_SwapChain_CONFIG_APPLE_CVPIXELBUFFER,
    filament_SwapChain_CONFIG_ENABLE_XCB, filament_SwapChain_CONFIG_READABLE,
    filament_SwapChain_CONFIG_TRANSPARENT, filament_SwapChain_getNativeWindow,
};

use crate::prelude::NativeHandle;

use super::Engine;

bitflags! {
    #[derive(Default)]
    pub struct SwapChainConfig: u64 {
        const TRANSPARENT = filament_SwapChain_CONFIG_TRANSPARENT;
        const READABLE = filament_SwapChain_CONFIG_READABLE;
        const ENABLE_XCB = filament_SwapChain_CONFIG_ENABLE_XCB;
        const APPLE_CVPIXELBUFFER = filament_SwapChain_CONFIG_APPLE_CVPIXELBUFFER;
    }
}

pub struct SwapChain(Rc<ptr::NonNull<filament_SwapChain>>, Engine);

impl NativeHandle<filament_SwapChain> for SwapChain {
    #[inline]
    fn native(&self) -> *const filament_SwapChain {
        self.0.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_SwapChain {
        self.0.as_ptr()
    }
}

impl SwapChain {
    #[inline]
    pub(crate) fn try_from_native(engine: Engine, native: *mut filament_SwapChain) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(SwapChain(Rc::new(ptr), engine))
    }

    #[inline]
    pub fn create_swap_chain(
        engine: &mut Engine,
        native_window: *mut raw::c_void,
        flags: SwapChainConfig,
    ) -> Option<SwapChain> {
        unsafe {
            SwapChain::try_from_native(
                engine.clone(),
                filament_Engine_createSwapChain(engine.native_mut(), native_window, flags.bits()),
            )
        }
    }

    #[inline]
    pub fn create_headless_swap_chain(
        engine: &mut Engine,
        width: u32,
        height: u32,
        flags: SwapChainConfig,
    ) -> Option<SwapChain> {
        unsafe {
            SwapChain::try_from_native(
                engine.clone(),
                filament_Engine_createSwapChain1(engine.native_mut(), width, height, flags.bits()),
            )
        }
    }

    #[inline]
    pub fn get_native_window(&self) -> *mut raw::c_void {
        unsafe { filament_SwapChain_getNativeWindow(self.native()) }
    }

    pub fn set_frame_scheduled_callback(&mut self, _: Option<fn()>) {
        todo!()
    }

    pub fn set_frame_completed_callback(&mut self, _: Option<fn()>) {
        todo!()
    }
}

impl Drop for SwapChain {
    #[inline]
    fn drop(&mut self) {
        if let Some(_) = Rc::get_mut(&mut self.0) {
            unsafe { filament_Engine_destroy15(self.1.native_mut(), self.native()) };
        }
    }
}
