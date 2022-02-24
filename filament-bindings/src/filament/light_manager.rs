use std::ptr;

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{bindgen, utils::Entity};

use super::Engine;

#[derive(IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(u8)]
pub enum LightType {
    SUN = bindgen::filament_LightManager_Type_SUN,
    DIRECTIONAL = bindgen::filament_LightManager_Type_DIRECTIONAL,
    POINT = bindgen::filament_LightManager_Type_POINT,
    FOCUSED_SPOT = bindgen::filament_LightManager_Type_FOCUSED_SPOT,
    SPOT = bindgen::filament_LightManager_Type_SPOT,
    #[num_enum(default)]
    UNKNOWN = u8::MAX,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct ShadowOption {
    pub map_size: u32,
    pub shadow_cascades: u8,
    pub cascade_split_positions: [f32; 3usize],
    pub constant_bias: f32,
    pub normal_bias: f32,
    pub shadow_far: f32,
    pub shadow_near_hint: f32,
    pub shadow_far_hint: f32,
    pub stable: bool,
    pub polygon_offset_constant: f32,
    pub polygon_offset_slope: f32,
    pub screen_space_contact_shadows: bool,
    pub step_count: u8,
    pub max_shadow_distance: f32,
    pub vsm: ShadowOptionVsm,
    pub shadow_bulb_radius: f32,
}

#[test]
fn shadow_option_layout_test() {
    assert_eq!(
        std::mem::size_of::<ShadowOption>(),
        std::mem::size_of::<bindgen::filament_LightManager_ShadowOptions>(),
    );
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct ShadowOptionVsm {
    pub msaa_samples: u8,
    pub blur_width: f32,
}

#[test]
fn shadow_option_vsm_layout_test() {
    assert_eq!(
        std::mem::size_of::<ShadowOptionVsm>(),
        std::mem::size_of::<bindgen::filament_LightManager_ShadowOptions_Vsm>(),
    );
}

pub mod ShadowCascades {
    use crate::bindgen;

    #[inline]
    pub unsafe fn compute_uniform_splits(cascades: u8) -> Vec<f32> {
        let mut split_positions = vec![0.0f32; cascades as usize - 1];
        bindgen::filament_LightManager_ShadowCascades_computeUniformSplits(
            split_positions.as_mut_ptr(),
            cascades,
        );
        split_positions
    }

    #[inline]
    pub unsafe fn compute_log_splits(cascades: u8, near: f32, far: f32) -> Vec<f32> {
        let mut split_positions = vec![0.0f32; cascades as usize - 1];
        bindgen::filament_LightManager_ShadowCascades_computeLogSplits(
            split_positions.as_mut_ptr(),
            cascades,
            near,
            far,
        );
        split_positions
    }

    #[inline]
    pub unsafe fn compute_practical_splits(
        cascades: u8,
        near: f32,
        far: f32,
        lambda: f32,
    ) -> Vec<f32> {
        let mut split_positions = vec![0.0f32; cascades as usize - 1];
        bindgen::filament_LightManager_ShadowCascades_computePracticalSplits(
            split_positions.as_mut_ptr(),
            cascades,
            near,
            far,
            lambda,
        );
        split_positions
    }
}

pub struct LightBuilder {
    native: bindgen::filament_LightManager_Builder,
}

impl LightBuilder {
    #[inline]
    pub unsafe fn lightChannel(&mut self, channel: u32, enable: bool) -> &mut Self {
        bindgen::filament_LightManager_Builder_lightChannel(&mut self.native, channel, enable);
        self
    }
    #[inline]
    pub unsafe fn castShadows(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_LightManager_Builder_castShadows(&mut self.native, enable);
        self
    }
    #[inline]
    pub unsafe fn shadowOptions(&mut self, options: &ShadowOption) -> &mut Self {
        bindgen::filament_LightManager_Builder_shadowOptions(
            &mut self.native,
            std::mem::transmute(options),
        );
        self
    }
    #[inline]
    pub unsafe fn castLight(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_LightManager_Builder_castLight(&mut self.native, enable);
        self
    }
    #[inline]
    pub unsafe fn position(&mut self, position: &bindgen::filament_math_float3) -> &mut Self {
        bindgen::filament_LightManager_Builder_position(&mut self.native, position);
        self
    }
    #[inline]
    pub unsafe fn direction(&mut self, direction: &bindgen::filament_math_float3) -> &mut Self {
        bindgen::filament_LightManager_Builder_direction(&mut self.native, direction);
        self
    }
    #[inline]
    pub unsafe fn color(&mut self, color: &bindgen::filament_LinearColor) -> &mut Self {
        bindgen::filament_LightManager_Builder_color(&mut self.native, color);
        self
    }
    #[inline]
    pub unsafe fn intensity(&mut self, intensity: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_intensity(&mut self.native, intensity);
        self
    }
    #[inline]
    pub unsafe fn intensityCandela(&mut self, intensity: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_intensityCandela(&mut self.native, intensity);
        self
    }
    #[inline]
    pub unsafe fn intensity_watts(&mut self, watts: f32, efficiency: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_intensity1(&mut self.native, watts, efficiency);
        self
    }
    #[inline]
    pub unsafe fn falloff(&mut self, radius: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_falloff(&mut self.native, radius);
        self
    }
    #[inline]
    pub unsafe fn spotLightCone(&mut self, inner: f32, outer: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_spotLightCone(&mut self.native, inner, outer);
        self
    }
    #[inline]
    pub unsafe fn sunAngularRadius(&mut self, angularRadius: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_sunAngularRadius(&mut self.native, angularRadius);
        self
    }
    #[inline]
    pub unsafe fn sunHaloSize(&mut self, haloSize: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_sunHaloSize(&mut self.native, haloSize);
        self
    }
    #[inline]
    pub unsafe fn sunHaloFalloff(&mut self, haloFalloff: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_sunHaloFalloff(&mut self.native, haloFalloff);
        self
    }
    #[inline]
    pub unsafe fn build(&mut self, engine: &mut Engine, entity: &Entity) -> Option<&mut Self> {
        if bindgen::filament_LightManager_Builder_build(
            &mut self.native,
            engine.native_mut(),
            entity.native_owned(),
        ) == bindgen::filament_LightManager_Builder_Result_Success
        {
            Some(self)
        } else {
            None
        }
    }
}

impl Drop for LightBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.native.destruct() }
    }
}

pub struct LightManager {
    native: ptr::NonNull<bindgen::filament_LightManager>,
}

impl LightManager {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_LightManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_LightManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_LightManager) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }
}

// TODO
