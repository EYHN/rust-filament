use std::{mem, ptr};

use bitflags::bitflags;

use filament_bindings::{
    filament_Engine_createRenderer, filament_Engine_destroy11, filament_Renderer,
    filament_Renderer_CLEAR, filament_Renderer_COMMIT, filament_Renderer_CopyFrameFlag,
    filament_Renderer_SET_PRESENTATION_TIME, filament_Renderer_beginFrame,
    filament_Renderer_copyFrame, filament_Renderer_endFrame, filament_Renderer_getUserTime,
    filament_Renderer_render, filament_Renderer_renderStandaloneView,
    filament_Renderer_resetUserTime, filament_Renderer_setClearOptions,
    filament_Renderer_setDisplayInfo, filament_Renderer_setFrameRateOptions, filament_math_float4,
};

use crate::prelude::{EngineDrop, EngineSystem, NativeHandle, RcHandle};

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
// TODO: #[derive(Debug, Clone)]
pub struct ClearOptions {
    pub clear_color: filament_math_float4,
    pub clear: bool,
    pub discard: bool,
}

bitflags! {
  #[derive(Default)]
  pub struct CopyFrameFlag: filament_Renderer_CopyFrameFlag {
    const COMMIT = filament_Renderer_COMMIT;
    const SET_PRESENTATION_TIME = filament_Renderer_SET_PRESENTATION_TIME;
    const CLEAR = filament_Renderer_CLEAR;
  }
}

pub struct RendererInner {
    native: ptr::NonNull<filament_Renderer>,
}

pub type Renderer = RcHandle<EngineSystem<RendererInner>>;

impl NativeHandle<filament_Renderer> for Renderer {
    #[inline]
    fn native(&self) -> *const filament_Renderer {
        self.data().native.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_Renderer {
        self.data_mut().native.as_ptr()
    }
}

impl Renderer {
    #[inline]
    pub(crate) fn try_from_native(engine: Engine, native: *mut filament_Renderer) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Renderer::new(EngineSystem::new(
            RendererInner { native: ptr },
            engine,
        )))
    }

    #[inline]
    pub fn create(engine: &mut Engine) -> Option<Renderer> {
        unsafe {
            Renderer::try_from_native(
                engine.clone(),
                filament_Engine_createRenderer(engine.native_mut()),
            )
        }
    }

    #[inline]
    pub fn set_display_info(&mut self, info: &DisplayInfo) {
        unsafe {
            filament_Renderer_setDisplayInfo(self.native_mut(), mem::transmute(info));
        }
    }

    #[inline]
    pub fn set_frame_rate_options(&mut self, options: &FrameRateOptions) {
        unsafe {
            filament_Renderer_setFrameRateOptions(self.native_mut(), mem::transmute(options));
        }
    }

    #[inline]
    pub fn set_clear_options(&mut self, options: &ClearOptions) {
        unsafe {
            filament_Renderer_setClearOptions(self.native_mut(), mem::transmute(options));
        }
    }

    #[inline]
    pub fn get_engine(&self) -> &Engine {
        &self.engine()
    }

    #[inline]
    pub fn begin_frame(&mut self, swap_chain: &mut SwapChain) -> bool {
        unsafe { filament_Renderer_beginFrame(self.native_mut(), swap_chain.native_mut(), 0) }
    }

    #[inline]
    pub fn begin_frame_vsync(
        &mut self,
        swap_chain: &mut SwapChain,
        vsync_steady_clock_time_nano: u64,
    ) -> bool {
        unsafe {
            filament_Renderer_beginFrame(
                self.native_mut(),
                swap_chain.native_mut(),
                vsync_steady_clock_time_nano,
            )
        }
    }

    #[inline]
    pub fn render(&mut self, view: &View) {
        unsafe { filament_Renderer_render(self.native_mut(), view.native()) }
    }

    #[inline]
    pub fn render_standalone_view(&mut self, view: &View) {
        unsafe { filament_Renderer_renderStandaloneView(self.native_mut(), view.native()) }
    }

    #[inline]
    pub fn copy_frame(
        &mut self,
        swap_chain: &mut SwapChain,
        dist_viewport: &Viewport,
        src_viewport: &Viewport,
        flags: CopyFrameFlag,
    ) {
        unsafe {
            filament_Renderer_copyFrame(
                self.native_mut(),
                swap_chain.native_mut(),
                dist_viewport.as_native() as *const _,
                src_viewport.as_native() as *const _,
                flags.bits(),
            )
        }
    }

    // TODO: read_pixels

    #[inline]
    pub fn end_frame(&mut self) {
        unsafe { filament_Renderer_endFrame(self.native_mut()) }
    }

    #[inline]
    pub fn get_user_time(&self) -> f64 {
        unsafe { filament_Renderer_getUserTime(self.native()) }
    }

    #[inline]
    pub fn reset_user_time(&mut self) {
        unsafe { filament_Renderer_resetUserTime(self.native_mut()) }
    }
}

impl EngineDrop for RendererInner {
    #[inline]
    fn drop(&mut self, engine: &mut Engine) {
        unsafe { filament_Engine_destroy11(engine.native_mut(), self.native.as_ptr()) };
    }
}
