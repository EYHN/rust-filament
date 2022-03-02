use std::{mem, ptr};

use bitflags::bitflags;

use crate::{bindgen, math::Float4, backend::PixelBufferDescriptor};

use super::{Engine, SwapChain, View, Viewport};

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DisplayInfo {
    pub refresh_rate: f32,
    pub presentation_deadline_nanos: u64,
    pub vsync_offset_nanos: u64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FrameRateOptions {
    pub head_room_ratio: f32,
    pub scale_rate: f32,
    pub history: u8,
    pub interval: u8,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClearOptions {
    pub clear_color: Float4,
    pub clear: bool,
    pub discard: bool,
}

bitflags! {
  #[derive(Default)]
  pub struct CopyFrameFlag: bindgen::filament_Renderer_CopyFrameFlag {
    const COMMIT = bindgen::filament_Renderer_COMMIT;
    const SET_PRESENTATION_TIME = bindgen::filament_Renderer_SET_PRESENTATION_TIME;
    const CLEAR = bindgen::filament_Renderer_CLEAR;
  }
}

pub struct Renderer {
    native: ptr::NonNull<bindgen::filament_Renderer>,
}

impl Renderer {
    #[inline]
    pub unsafe fn native(&self) -> *const bindgen::filament_Renderer {
        self.native.as_ptr()
    }

    #[inline]
    pub unsafe fn native_mut(&mut self) -> *mut bindgen::filament_Renderer {
        self.native.as_ptr()
    }

    #[inline]
    pub unsafe fn try_from_native(native: *mut bindgen::filament_Renderer) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Renderer { native: ptr })
    }

    #[inline]
    pub unsafe fn set_display_info(&mut self, info: &DisplayInfo) {
        bindgen::filament_Renderer_setDisplayInfo(self.native_mut(), mem::transmute(info));
    }

    #[inline]
    pub unsafe fn set_frame_rate_options(&mut self, options: &FrameRateOptions) {
        bindgen::filament_Renderer_setFrameRateOptions(self.native_mut(), mem::transmute(options));
    }

    #[inline]
    pub unsafe fn set_clear_options(&mut self, options: &ClearOptions) {
        bindgen::filament_Renderer_setClearOptions(self.native_mut(), mem::transmute(options));
    }

    #[inline]
    pub unsafe fn get_engine(&mut self) -> Option<Engine> {
        Engine::try_from_native(bindgen::filament_Renderer_getEngine(self.native_mut()))
    }

    #[inline]
    pub unsafe fn begin_frame(&mut self, swap_chain: &mut SwapChain) -> bool {
        bindgen::filament_Renderer_beginFrame(self.native_mut(), swap_chain.native_mut(), 0)
    }

    #[inline]
    pub unsafe fn begin_frame_vsync(
        &mut self,
        swap_chain: &mut SwapChain,
        vsync_steady_clock_time_nano: u64,
    ) -> bool {
        bindgen::filament_Renderer_beginFrame(
            self.native_mut(),
            swap_chain.native_mut(),
            vsync_steady_clock_time_nano,
        )
    }

    #[inline]
    pub unsafe fn render(&mut self, view: &View) {
        bindgen::filament_Renderer_render(self.native_mut(), view.native())
    }

    #[inline]
    pub unsafe fn render_standalone_view(&mut self, view: &View) {
        bindgen::filament_Renderer_renderStandaloneView(self.native_mut(), view.native())
    }

    #[inline]
    pub unsafe fn copy_frame(
        &mut self,
        swap_chain: &mut SwapChain,
        dist_viewport: &Viewport,
        src_viewport: &Viewport,
        flags: CopyFrameFlag,
    ) {
        bindgen::filament_Renderer_copyFrame(
            self.native_mut(),
            swap_chain.native_mut(),
            dist_viewport.as_native() as *const _,
            src_viewport.as_native() as *const _,
            flags.bits(),
        )
    }

    #[inline]
    pub unsafe fn read_pixels(
        &mut self,
        xoffset: u32,
        yoffset: u32,
        width: u32,
        height: u32,
        buffer: PixelBufferDescriptor<u8>,
    ) {
        bindgen::filament_Renderer_readPixels(
            self.native_mut(),
            xoffset,
            yoffset,
            width,
            height,
            &mut buffer.into_native(),
        )
    }

    #[inline]
    pub unsafe fn end_frame(&mut self) {
        bindgen::filament_Renderer_endFrame(self.native_mut())
    }

    #[inline]
    pub unsafe fn get_user_time(&self) -> f64 {
        bindgen::filament_Renderer_getUserTime(self.native())
    }

    #[inline]
    pub unsafe fn reset_user_time(&mut self) {
        bindgen::filament_Renderer_resetUserTime(self.native_mut())
    }
}
