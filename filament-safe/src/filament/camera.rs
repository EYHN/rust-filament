use std::{ptr};

use filament_bindings::{
    filament_Camera, filament_Camera_Fov_HORIZONTAL, filament_Camera_Fov_VERTICAL,
    filament_Camera_Projection_ORTHO, filament_Camera_Projection_PERSPECTIVE,
    filament_Camera_computeEffectiveFocalLength, filament_Camera_computeEffectiveFov,
    filament_Camera_getAperture, filament_Camera_getCullingFar,
    filament_Camera_getCullingProjectionMatrix, filament_Camera_getFieldOfViewInDegrees,
    filament_Camera_getFocalLength, filament_Camera_getFocusDistance,
    filament_Camera_getForwardVector, filament_Camera_getFrustum, filament_Camera_getLeftVector,
    filament_Camera_getModelMatrix, filament_Camera_getNear, filament_Camera_getPosition,
    filament_Camera_getProjectionMatrix, filament_Camera_getScaling,
    filament_Camera_getSensitivity, filament_Camera_getShift, filament_Camera_getShutterSpeed,
    filament_Camera_getUpVector, filament_Camera_getViewMatrix, filament_Camera_inverseProjection,
    filament_Camera_inverseProjection1, filament_Camera_lookAt, filament_Camera_lookAt1,
    filament_Camera_setCustomProjection, filament_Camera_setCustomProjection1,
    filament_Camera_setExposure, filament_Camera_setFocusDistance,
    filament_Camera_setLensProjection, filament_Camera_setModelMatrix,
    filament_Camera_setModelMatrix1, filament_Camera_setProjection, filament_Camera_setProjection1,
    filament_Camera_setScaling, filament_Camera_setShift, filament_Frustum, filament_math_double2,
    filament_math_double4, filament_math_float3, filament_math_mat4, filament_math_mat4f,
};
use num_enum::IntoPrimitive;

use crate::{prelude::NativeHandle};

#[derive(IntoPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(i32)]
pub enum Projection {
    PERSPECTIVE = filament_Camera_Projection_PERSPECTIVE,
    ORTHO = filament_Camera_Projection_ORTHO,
}

#[derive(IntoPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(i32)]
pub enum Fov {
    VERTICAL = filament_Camera_Fov_VERTICAL,
    HORIZONTAL = filament_Camera_Fov_HORIZONTAL,
}

pub struct Camera(ptr::NonNull<filament_Camera>);

impl NativeHandle<filament_Camera> for Camera {
    #[inline]
    fn native(&self) -> *const filament_Camera {
        self.0.as_ptr()
    }

    #[inline]
    fn native_mut(&mut self) -> *mut filament_Camera {
        self.0.as_ptr()
    }
}

impl Camera {
    #[inline]
    pub(crate) fn try_from_native(native: *mut filament_Camera) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Camera(ptr))
    }

    #[inline]
    pub fn set_projection(
        &mut self,
        projection: Projection,
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) {
        unsafe {
            filament_Camera_setProjection(
                self.native_mut(),
                projection.into(),
                left,
                right,
                bottom,
                top,
                near,
                far,
            )
        }
    }

    #[inline]
    pub fn set_projection_fov(&mut self, fov_in_degrees: f64, aspect: f64, near: f64, far: f64) {
        unsafe {
            filament_Camera_setProjection1(
                self.native_mut(),
                fov_in_degrees,
                aspect,
                near,
                far,
                Fov::VERTICAL.into(),
            )
        }
    }

    #[inline]
    pub fn set_projection_fov_direction(
        &mut self,
        fov_in_degrees: f64,
        aspect: f64,
        near: f64,
        far: f64,
        direction: Fov,
    ) {
        unsafe {
            filament_Camera_setProjection1(
                self.native_mut(),
                fov_in_degrees,
                aspect,
                near,
                far,
                direction.into(),
            )
        }
    }

    #[inline]
    pub fn set_lens_projection(
        &mut self,
        focal_length_in_millimeters: f64,
        aspect: f64,
        near: f64,
        far: f64,
    ) {
        unsafe {
            filament_Camera_setLensProjection(
                self.native_mut(),
                focal_length_in_millimeters,
                aspect,
                near,
                far,
            )
        }
    }

    #[inline]
    pub fn set_custom_projection(&mut self, projection: &filament_math_mat4, near: f64, far: f64) {
        unsafe { filament_Camera_setCustomProjection(self.native_mut(), projection, near, far) }
    }

    #[inline]
    pub fn set_custom_projection_culling(
        &mut self,
        projection: &filament_math_mat4,
        projection_for_culling: &filament_math_mat4,
        near: f64,
        far: f64,
    ) {
        unsafe {
            filament_Camera_setCustomProjection1(
                self.native_mut(),
                projection,
                projection_for_culling,
                near,
                far,
            )
        }
    }

    #[inline]
    pub fn set_scaling(&mut self, scaling: filament_math_double2) {
        unsafe { filament_Camera_setScaling(self.native_mut(), scaling) }
    }

    #[inline]
    pub fn set_shift(&mut self, shift: filament_math_double2) {
        unsafe { filament_Camera_setShift(self.native_mut(), shift) }
    }

    #[inline]
    pub fn get_scaling(&self) -> filament_math_double4 {
        unsafe { filament_Camera_getScaling(self.native()) }
    }

    #[inline]
    pub fn get_shift(&self) -> filament_math_double2 {
        unsafe { filament_Camera_getShift(self.native()) }
    }

    #[inline]
    pub fn get_projection_matrix(&self) -> filament_math_mat4 {
        unsafe { filament_Camera_getProjectionMatrix(self.native()) }
    }

    #[inline]
    pub fn get_culling_projection_matrix(&self) -> filament_math_mat4 {
        unsafe { filament_Camera_getCullingProjectionMatrix(self.native()) }
    }

    #[inline]
    pub fn get_near(&self) -> f32 {
        unsafe { filament_Camera_getNear(self.native()) }
    }

    #[inline]
    pub fn get_culling_far(&self) -> f32 {
        unsafe { filament_Camera_getCullingFar(self.native()) }
    }

    #[inline]
    pub fn set_model_matrix(&mut self, view: &filament_math_mat4) {
        unsafe { filament_Camera_setModelMatrix(self.native_mut(), view) }
    }

    #[inline]
    pub fn set_model_matrix_float(&mut self, view: &filament_math_mat4f) {
        unsafe { filament_Camera_setModelMatrix1(self.native_mut(), view) }
    }

    #[inline]
    pub fn look_at_up(
        &mut self,
        eye: &filament_math_float3,
        center: &filament_math_float3,
        up: &filament_math_float3,
    ) {
        unsafe { filament_Camera_lookAt(self.native_mut(), eye, center, up) }
    }

    #[inline]
    pub fn look_at(&mut self, eye: &filament_math_float3, center: &filament_math_float3) {
        unsafe { filament_Camera_lookAt1(self.native_mut(), eye, center) }
    }

    #[inline]
    pub fn get_model_matrix(&self) -> filament_math_mat4 {
        unsafe { filament_Camera_getModelMatrix(self.native()) }
    }

    #[inline]
    pub fn get_view_matrix(&self) -> filament_math_mat4 {
        unsafe { filament_Camera_getViewMatrix(self.native()) }
    }

    #[inline]
    pub fn get_position(&self) -> filament_math_float3 {
        unsafe { filament_Camera_getPosition(self.native()) }
    }

    #[inline]
    pub fn get_left_vector(&self) -> filament_math_float3 {
        unsafe { filament_Camera_getLeftVector(self.native()) }
    }

    #[inline]
    pub fn get_up_vector(&self) -> filament_math_float3 {
        unsafe { filament_Camera_getUpVector(self.native()) }
    }

    #[inline]
    pub fn get_forward_vector(&self) -> filament_math_float3 {
        unsafe { filament_Camera_getForwardVector(self.native()) }
    }

    #[inline]
    pub fn get_field_of_view_in_degrees(&self, direction: Fov) -> f32 {
        unsafe { filament_Camera_getFieldOfViewInDegrees(self.native(), direction.into()) }
    }

    #[inline]
    pub fn get_frustum(&self) -> filament_Frustum {
        unsafe { filament_Camera_getFrustum(self.native()) }
    }

    #[inline]
    pub fn set_exposure(&mut self, exposure: f32) {
        self.set_exposure_physical(1.0, 1.2, 100.0 * (1.0 / exposure))
    }

    #[inline]
    pub fn set_exposure_physical(&mut self, aperture: f32, shutter_speed: f32, sensitivity: f32) {
        unsafe {
            filament_Camera_setExposure(self.native_mut(), aperture, shutter_speed, sensitivity)
        }
    }

    #[inline]
    pub fn get_aperture(&self) -> f32 {
        unsafe { filament_Camera_getAperture(self.native()) }
    }

    #[inline]
    pub fn get_shutter_speed(&self) -> f32 {
        unsafe { filament_Camera_getShutterSpeed(self.native()) }
    }

    #[inline]
    pub fn get_sensitivity(&self) -> f32 {
        unsafe { filament_Camera_getSensitivity(self.native()) }
    }

    #[inline]
    pub fn get_focal_length(&self) -> f64 {
        unsafe { filament_Camera_getFocalLength(self.native()) }
    }

    #[inline]
    pub fn set_focus_distance(&mut self, distance: f32) {
        unsafe { filament_Camera_setFocusDistance(self.native_mut(), distance) }
    }

    #[inline]
    pub fn get_focus_distance(&self) -> f32 {
        unsafe { filament_Camera_getFocusDistance(self.native()) }
    }

    #[inline]
    pub fn inverse_projection(p: &filament_math_mat4) -> filament_math_mat4 {
        unsafe { filament_Camera_inverseProjection(p) }
    }

    #[inline]
    pub fn inverse_projection_float(p: &filament_math_mat4f) -> filament_math_mat4f {
        unsafe { filament_Camera_inverseProjection1(p) }
    }

    #[inline]
    pub fn compute_effective_focal_length(focal_length: f64, focus_distance: f64) -> f64 {
        unsafe { filament_Camera_computeEffectiveFocalLength(focal_length, focus_distance) }
    }

    #[inline]
    pub fn compute_effective_fov(fov_in_degrees: f64, focus_distance: f64) -> f64 {
        unsafe { filament_Camera_computeEffectiveFov(fov_in_degrees, focus_distance) }
    }
}
