use filament_bindings::{
    filament_TextureSampler, filament_backend_SamplerParams,
    filament_backend_SamplerParams__bindgen_ty_1,
    filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1,
};

use crate::backend::{
    SamplerCompareFunc, SamplerCompareMode, SamplerMagFilter, SamplerMinFilter, SamplerWrapMode,
};

pub struct TextureSampler {
    native: filament_TextureSampler,
}

impl TextureSampler {
    #[inline]
    pub fn new(
        filter_mag: SamplerMagFilter,
        filter_min: SamplerMinFilter,
        wrap_s: SamplerWrapMode,
        wrap_t: SamplerWrapMode,
        wrap_r: SamplerWrapMode,
        anisotropy_log2: u8,
        compare_mode: SamplerCompareMode,
        padding0: u8,
        compare_func: SamplerCompareFunc,
        padding1: u8,
        padding2: u8,
    ) -> Self {
        Self {
            native: filament_TextureSampler {
                mSamplerParams: filament_backend_SamplerParams {
                    __bindgen_anon_1: filament_backend_SamplerParams__bindgen_ty_1 {
                        __bindgen_anon_1: filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1 {
                            _bitfield_1: filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1::new_bitfield_1(
                                filter_mag.into(),
                                filter_min.into(),
                                wrap_s.into(),
                                wrap_t.into(),
                                wrap_r.into(),
                                anisotropy_log2,
                                compare_mode.into(),
                                padding0,
                                compare_func.into(),
                                padding1,
                                padding2,
                            ),
                            ..Default::default()
                        }
                    }
                },
            },
        }
    }

    #[inline]
    pub(crate) fn native(
        &self,
    ) -> &filament_TextureSampler {
        &self.native
    }

    #[inline]
    pub(crate) fn native_params(
        &self,
    ) -> filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1 {
        unsafe { self.native.mSamplerParams.__bindgen_anon_1.__bindgen_anon_1 }
    }

    #[inline]
    pub fn filter_mag(&self) -> SamplerMagFilter {
        SamplerMagFilter::from(self.native_params().filterMag())
    }

    #[inline]
    pub fn set_filter_mag(&mut self, val: SamplerMagFilter) {
        self.native_params().set_filterMag(val.into())
    }

    #[inline]
    pub fn filter_min(&self) -> SamplerMinFilter {
        SamplerMinFilter::from(self.native_params().filterMin())
    }

    #[inline]
    pub fn set_filter_min(&mut self, val: SamplerMinFilter) {
        self.native_params().set_filterMin(val.into())
    }

    #[inline]
    pub fn wrap_s(&self) -> SamplerWrapMode {
        SamplerWrapMode::from(self.native_params().wrapS())
    }

    #[inline]
    pub fn set_wrap_s(&mut self, val: SamplerWrapMode) {
        self.native_params().set_wrapS(val.into())
    }

    #[inline]
    pub fn wrap_tt(&self) -> SamplerWrapMode {
        SamplerWrapMode::from(self.native_params().wrapT())
    }

    #[inline]
    pub fn set_wrap_t(&mut self, val: SamplerWrapMode) {
        self.native_params().set_wrapT(val.into())
    }

    #[inline]
    pub fn wrap_r(&self) -> SamplerWrapMode {
        SamplerWrapMode::from(self.native_params().wrapR())
    }

    #[inline]
    pub fn set_wrap_r(&mut self, val: SamplerWrapMode) {
        self.native_params().set_wrapR(val.into())
    }

    #[inline]
    pub fn anisotropy_log2(&self) -> u8 {
        self.native_params().anisotropyLog2()
    }

    #[inline]
    pub fn set_anisotropy_log2(&mut self, val: u8) {
        self.native_params().set_anisotropyLog2(val)
    }

    #[inline]
    pub fn compare_mode(&self) -> SamplerCompareMode {
        SamplerCompareMode::from(self.native_params().compareMode())
    }

    #[inline]
    pub fn set_compare_mode(&mut self, val: SamplerCompareMode) {
        self.native_params().set_compareMode(val.into())
    }

    #[inline]
    pub fn padding0(&self) -> u8 {
        self.native_params().padding0()
    }

    #[inline]
    pub fn set_padding0(&mut self, val: u8) {
        self.native_params().set_padding0(val)
    }

    #[inline]
    pub fn compare_func(&self) -> SamplerCompareFunc {
        SamplerCompareFunc::from(self.native_params().compareFunc())
    }

    #[inline]
    pub fn set_compare_func(&mut self, val: SamplerCompareFunc) {
        self.native_params().set_compareFunc(val.into())
    }

    #[inline]
    pub fn padding1(&self) -> u8 {
        self.native_params().padding1()
    }

    #[inline]
    pub fn set_padding1(&mut self, val: u8) {
        self.native_params().set_padding1(val)
    }

    #[inline]
    pub fn padding2(&self) -> u8 {
        self.native_params().padding2()
    }

    #[inline]
    pub fn set_padding2(&mut self, val: u8) {
        self.native_params().set_padding2(val)
    }
}

impl Default for TextureSampler {
    fn default() -> Self {
        Self::new(
            SamplerMagFilter::NEAREST,
            SamplerMinFilter::NEAREST,
            SamplerWrapMode::CLAMP_TO_EDGE,
            SamplerWrapMode::CLAMP_TO_EDGE,
            SamplerWrapMode::CLAMP_TO_EDGE,
            0,
            SamplerCompareMode::NONE,
            0,
            SamplerCompareFunc::LE,
            0,
            0,
        )
    }
}
