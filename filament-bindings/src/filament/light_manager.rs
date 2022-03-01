#![allow(non_camel_case_types)]

use core::slice;
use std::ptr;

use num_enum::{FromPrimitive, IntoPrimitive};

use crate::{bindgen, utils::Entity, math::Float3};

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

pub mod shadow_cascades {
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
    pub unsafe fn new(light_type: LightType) -> Self {
        LightBuilder {
            native: bindgen::filament_LightManager_Builder::new(light_type.into()),
        }
    }
    #[inline]
    pub unsafe fn light_channel(&mut self, channel: u32, enable: bool) -> &mut Self {
        bindgen::filament_LightManager_Builder_lightChannel(&mut self.native, channel, enable);
        self
    }
    #[inline]
    pub unsafe fn cast_shadows(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_LightManager_Builder_castShadows(&mut self.native, enable);
        self
    }
    #[inline]
    pub unsafe fn shadow_options(&mut self, options: &ShadowOption) -> &mut Self {
        bindgen::filament_LightManager_Builder_shadowOptions(
            &mut self.native,
            options as *const _ as *const _,
        );
        self
    }
    #[inline]
    pub unsafe fn cast_light(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_LightManager_Builder_castLight(&mut self.native, enable);
        self
    }
    #[inline]
    pub unsafe fn position(&mut self, position: &Float3) -> &mut Self {
        bindgen::filament_LightManager_Builder_position(&mut self.native, position.native_ptr());
        self
    }
    #[inline]
    pub unsafe fn direction(&mut self, direction: &Float3) -> &mut Self {
        bindgen::filament_LightManager_Builder_direction(&mut self.native, direction.native_ptr());
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
    pub unsafe fn intensity_candela(&mut self, intensity: f32) -> &mut Self {
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
    pub unsafe fn spot_light_cone(&mut self, inner: f32, outer: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_spotLightCone(&mut self.native, inner, outer);
        self
    }
    #[inline]
    pub unsafe fn sun_angular_radius(&mut self, angular_radius: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_sunAngularRadius(&mut self.native, angular_radius);
        self
    }
    #[inline]
    pub unsafe fn sun_halo_size(&mut self, halo_size: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_sunHaloSize(&mut self.native, halo_size);
        self
    }
    #[inline]
    pub unsafe fn sun_halo_falloff(&mut self, halo_falloff: f32) -> &mut Self {
        bindgen::filament_LightManager_Builder_sunHaloFalloff(&mut self.native, halo_falloff);
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

#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct LightManagerInstance {
    native: bindgen::filament_LightManager_Instance,
}

impl LightManagerInstance {
    #[inline]
    pub fn native_ptr(&self) -> *const bindgen::filament_LightManager_Instance {
        &self.native
    }

    #[inline]
    pub fn native_ptr_mut(&mut self) -> *mut bindgen::filament_LightManager_Instance {
        &mut self.native
    }

    #[inline]
    pub(crate) fn native_owned(&self) -> bindgen::filament_LightManager_Instance {
        self.native.clone()
    }
}

impl LightManager {
    #[inline]
    pub unsafe fn get_entities(&self) -> &[Entity] {
        let len = bindgen::filament_LightManager_getComponentCount(self.native());
        let ptr = bindgen::filament_LightManager_getEntities(self.native());
        slice::from_raw_parts(ptr as *const _, len)
    }
    #[inline]
    pub unsafe fn has_component(&self, e: &Entity) -> bool {
        bindgen::filament_LightManager_hasComponent(self.native(), e.native_owned())
    }
    #[inline]
    pub unsafe fn get_instance(&self, e: &Entity) -> LightManagerInstance {
        LightManagerInstance {
            native: bindgen::filament_LightManager_getInstance(self.native(), e.native_owned()),
        }
    }
    #[inline]
    pub unsafe fn destroy(&mut self, e: &Entity) {
        bindgen::filament_LightManager_destroy(self.native_mut(), e.native_owned())
    }
    #[inline]
    pub unsafe fn get_type(&self, i: &LightManagerInstance) -> LightType {
        LightType::from(bindgen::filament_LightManager_getType(
            self.native(),
            i.native_owned(),
        ))
    }
    #[inline]
    pub unsafe fn set_light_channel(
        &mut self,
        i: &LightManagerInstance,
        channel: ::std::os::raw::c_uint,
        enable: bool,
    ) {
        bindgen::filament_LightManager_setLightChannel(
            self.native_mut(),
            i.native_owned(),
            channel,
            enable,
        )
    }
    #[inline]
    pub unsafe fn get_light_channel(
        &self,
        i: &LightManagerInstance,
        channel: ::std::os::raw::c_uint,
    ) -> bool {
        bindgen::filament_LightManager_getLightChannel(self.native(), i.native_owned(), channel)
    }
    #[inline]
    pub unsafe fn set_position(
        &mut self,
        i: &LightManagerInstance,
        position: &bindgen::filament_math_float3,
    ) {
        bindgen::filament_LightManager_setPosition(self.native_mut(), i.native_owned(), position)
    }
    #[inline]
    pub unsafe fn get_position(&self, i: &LightManagerInstance) -> &bindgen::filament_math_float3 {
        &*bindgen::filament_LightManager_getPosition(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn set_direction(
        &mut self,
        i: &LightManagerInstance,
        direction: &bindgen::filament_math_float3,
    ) {
        bindgen::filament_LightManager_setDirection(self.native_mut(), i.native_owned(), direction)
    }
    #[inline]
    pub unsafe fn get_direction(&self, i: &LightManagerInstance) -> &bindgen::filament_math_float3 {
        &*bindgen::filament_LightManager_getDirection(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn set_color(
        &mut self,
        i: &LightManagerInstance,
        color: &bindgen::filament_LinearColor,
    ) {
        bindgen::filament_LightManager_setColor(self.native_mut(), i.native_owned(), color)
    }
    #[inline]
    pub unsafe fn get_color(&self, i: &LightManagerInstance) -> &bindgen::filament_math_float3 {
        &*bindgen::filament_LightManager_getColor(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn set_intensity(&mut self, i: &LightManagerInstance, intensity: f32) {
        bindgen::filament_LightManager_setIntensity(self.native_mut(), i.native_owned(), intensity)
    }
    #[inline]
    pub unsafe fn set_intensity_candela(&mut self, i: &LightManagerInstance, intensity: f32) {
        bindgen::filament_LightManager_setIntensityCandela(
            self.native_mut(),
            i.native_owned(),
            intensity,
        )
    }
    #[inline]
    pub unsafe fn get_intensity(&self, i: &LightManagerInstance) -> f32 {
        bindgen::filament_LightManager_getIntensity(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn set_falloff(&mut self, i: &LightManagerInstance, radius: f32) {
        bindgen::filament_LightManager_setFalloff(self.native_mut(), i.native_owned(), radius)
    }
    #[inline]
    pub unsafe fn get_falloff(&self, i: &LightManagerInstance) -> f32 {
        bindgen::filament_LightManager_getFalloff(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn set_spot_light_cone(&mut self, i: &LightManagerInstance, inner: f32, outer: f32) {
        bindgen::filament_LightManager_setSpotLightCone(
            self.native_mut(),
            i.native_owned(),
            inner,
            outer,
        )
    }
    #[inline]
    pub unsafe fn get_spot_light_outer_cone(&self, i: &LightManagerInstance) -> f32 {
        bindgen::filament_LightManager_getSpotLightOuterCone(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn get_spot_light_inner_cone(&self, i: &LightManagerInstance) -> f32 {
        bindgen::filament_LightManager_getSpotLightInnerCone(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn set_sun_angular_radius(&mut self, i: &LightManagerInstance, angular_radius: f32) {
        bindgen::filament_LightManager_setSunAngularRadius(
            self.native_mut(),
            i.native_owned(),
            angular_radius,
        )
    }
    #[inline]
    pub unsafe fn get_sun_angular_radius(&self, i: &LightManagerInstance) -> f32 {
        bindgen::filament_LightManager_getSunAngularRadius(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn set_sun_halo_size(&mut self, i: &LightManagerInstance, halo_size: f32) {
        bindgen::filament_LightManager_setSunHaloSize(
            self.native_mut(),
            i.native_owned(),
            halo_size,
        )
    }
    #[inline]
    pub unsafe fn get_sun_halo_size(&self, i: &LightManagerInstance) -> f32 {
        bindgen::filament_LightManager_getSunHaloSize(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn set_sun_halo_falloff(&mut self, i: &LightManagerInstance, halo_falloff: f32) {
        bindgen::filament_LightManager_setSunHaloFalloff(
            self.native_mut(),
            i.native_owned(),
            halo_falloff,
        )
    }
    #[inline]
    pub unsafe fn get_sun_halo_falloff(&self, i: &LightManagerInstance) -> f32 {
        bindgen::filament_LightManager_getSunHaloFalloff(self.native(), i.native_owned())
    }
    #[inline]
    pub unsafe fn get_shadow_options(&self, i: &LightManagerInstance) -> &ShadowOption {
        &*(bindgen::filament_LightManager_getShadowOptions(self.native(), i.native_owned())
            as *const ShadowOption)
    }
    #[inline]
    pub unsafe fn set_shadow_options(&mut self, i: &LightManagerInstance, options: &ShadowOption) {
        bindgen::filament_LightManager_setShadowOptions(
            self.native_mut(),
            i.native_owned(),
            options as *const _ as *const _,
        )
    }
    #[inline]
    pub unsafe fn set_shadow_caster(&mut self, i: &LightManagerInstance, shadow_caster: bool) {
        bindgen::filament_LightManager_setShadowCaster(
            self.native_mut(),
            i.native_owned(),
            shadow_caster,
        )
    }
    #[inline]
    pub unsafe fn is_shadow_caster(&self, i: &LightManagerInstance) -> bool {
        bindgen::filament_LightManager_isShadowCaster(self.native(), i.native_owned())
    }
}
