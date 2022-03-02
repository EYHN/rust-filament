use std::ptr;

use num_enum::IntoPrimitive;

use crate::{bindgen, math::Float3};

#[derive(IntoPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(i32)]
pub enum Projection {
    PERSPECTIVE = bindgen::filament_Camera_Projection_PERSPECTIVE,
    ORTHO = bindgen::filament_Camera_Projection_ORTHO,
}

#[derive(IntoPrimitive, Clone, Copy, PartialEq, PartialOrd, Debug)]
#[repr(i32)]
pub enum Fov {
    VERTICAL = bindgen::filament_Camera_Fov_VERTICAL,
    HORIZONTAL = bindgen::filament_Camera_Fov_HORIZONTAL,
}

#[repr(transparent)]
pub struct Camera {
    native: ptr::NonNull<bindgen::filament_Camera>,
}

impl Camera {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_Camera {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_Camera {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_Camera) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Camera { native: ptr })
    }

    #[inline]
    pub unsafe fn set_projection(
        &mut self,
        projection: Projection,
        left: f64,
        right: f64,
        bottom: f64,
        top: f64,
        near: f64,
        far: f64,
    ) {
        bindgen::filament_Camera_setProjection(
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

    #[inline]
    pub unsafe fn set_projection_fov(
        &mut self,
        fov_in_degrees: f64,
        aspect: f64,
        near: f64,
        far: f64,
    ) {
        bindgen::filament_Camera_setProjection1(
            self.native_mut(),
            fov_in_degrees,
            aspect,
            near,
            far,
            Fov::VERTICAL.into(),
        )
    }

    #[inline]
    pub unsafe fn set_projection_fov_direction(
        &mut self,
        fov_in_degrees: f64,
        aspect: f64,
        near: f64,
        far: f64,
        direction: Fov,
    ) {
        bindgen::filament_Camera_setProjection1(
            self.native_mut(),
            fov_in_degrees,
            aspect,
            near,
            far,
            direction.into(),
        )
    }

    #[inline]
    pub unsafe fn set_lens_projection(
        &mut self,
        focal_length_in_millimeters: f64,
        aspect: f64,
        near: f64,
        far: f64,
    ) {
        bindgen::filament_Camera_setLensProjection(
            self.native_mut(),
            focal_length_in_millimeters,
            aspect,
            near,
            far,
        )
    }

    #[inline]
    pub unsafe fn set_custom_projection(
        &mut self,
        projection: &bindgen::filament_math_mat4,
        near: f64,
        far: f64,
    ) {
        bindgen::filament_Camera_setCustomProjection(self.native_mut(), projection, near, far)
    }

    #[inline]
    pub unsafe fn set_custom_projection_culling(
        &mut self,
        projection: &bindgen::filament_math_mat4,
        projection_for_culling: &bindgen::filament_math_mat4,
        near: f64,
        far: f64,
    ) {
        bindgen::filament_Camera_setCustomProjection1(
            self.native_mut(),
            projection,
            projection_for_culling,
            near,
            far,
        )
    }

    #[inline]
    pub unsafe fn set_scaling(&mut self, scaling: bindgen::filament_math_double2) {
        bindgen::filament_Camera_setScaling(self.native_mut(), scaling)
    }

    #[inline]
    pub unsafe fn set_shift(&mut self, shift: bindgen::filament_math_double2) {
        bindgen::filament_Camera_setShift(self.native_mut(), shift)
    }

    #[inline]
    pub unsafe fn get_scaling(&self) -> bindgen::filament_math_double4 {
        bindgen::filament_Camera_getScaling(self.native())
    }

    #[inline]
    pub unsafe fn get_shift(&self) -> bindgen::filament_math_double2 {
        bindgen::filament_Camera_getShift(self.native())
    }

    #[inline]
    pub unsafe fn get_projection_matrix(&self) -> bindgen::filament_math_mat4 {
        bindgen::filament_Camera_getProjectionMatrix(self.native())
    }

    #[inline]
    pub unsafe fn get_culling_projection_matrix(&self) -> bindgen::filament_math_mat4 {
        bindgen::filament_Camera_getCullingProjectionMatrix(self.native())
    }

    #[inline]
    pub unsafe fn get_near(&self) -> f32 {
        bindgen::filament_Camera_getNear(self.native())
    }

    #[inline]
    pub unsafe fn get_culling_far(&self) -> f32 {
        bindgen::filament_Camera_getCullingFar(self.native())
    }

    #[inline]
    pub unsafe fn set_model_matrix(&mut self, view: &bindgen::filament_math_mat4) {
        bindgen::filament_Camera_setModelMatrix(self.native_mut(), view)
    }

    #[inline]
    pub unsafe fn set_model_matrix_float(&mut self, view: &bindgen::filament_math_mat4f) {
        bindgen::filament_Camera_setModelMatrix1(self.native_mut(), view)
    }

    #[inline]
    pub unsafe fn look_at_up(&mut self, eye: &Float3, center: &Float3, up: &Float3) {
        bindgen::filament_Camera_lookAt(
            self.native_mut(),
            eye.native_ptr(),
            center.native_ptr(),
            up.native_ptr(),
        )
    }

    #[inline]
    pub unsafe fn look_at(&mut self, eye: &Float3, center: &Float3) {
        bindgen::filament_Camera_lookAt1(self.native_mut(), eye.native_ptr(), center.native_ptr())
    }

    #[inline]
    pub unsafe fn get_model_matrix(&self) -> bindgen::filament_math_mat4 {
        bindgen::filament_Camera_getModelMatrix(self.native())
    }

    #[inline]
    pub unsafe fn get_view_matrix(&self) -> bindgen::filament_math_mat4 {
        bindgen::filament_Camera_getViewMatrix(self.native())
    }

    #[inline]
    pub unsafe fn get_position(&self) -> Float3 {
        Float3::from_native(bindgen::filament_Camera_getPosition(self.native()))
    }

    #[inline]
    pub unsafe fn get_left_vector(&self) -> Float3 {
        Float3::from_native(bindgen::filament_Camera_getLeftVector(self.native()))
    }

    #[inline]
    pub unsafe fn get_up_vector(&self) -> Float3 {
        Float3::from_native(bindgen::filament_Camera_getUpVector(self.native()))
    }

    #[inline]
    pub unsafe fn get_forward_vector(&self) -> Float3 {
        Float3::from_native(bindgen::filament_Camera_getForwardVector(self.native()))
    }

    #[inline]
    pub unsafe fn get_field_of_view_in_degrees(&self, direction: Fov) -> f32 {
        bindgen::filament_Camera_getFieldOfViewInDegrees(self.native(), direction.into())
    }

    #[inline]
    pub unsafe fn get_frustum(&self) -> bindgen::filament_Frustum {
        bindgen::filament_Camera_getFrustum(self.native())
    }

    #[inline]
    pub unsafe fn set_exposure(&mut self, exposure: f32) {
        self.set_exposure_physical(1.0, 1.2, 100.0 * (1.0 / exposure))
    }

    #[inline]
    pub unsafe fn set_exposure_physical(
        &mut self,
        aperture: f32,
        shutter_speed: f32,
        sensitivity: f32,
    ) {
        bindgen::filament_Camera_setExposure(
            self.native_mut(),
            aperture,
            shutter_speed,
            sensitivity,
        )
    }

    #[inline]
    pub unsafe fn get_aperture(&self) -> f32 {
        bindgen::filament_Camera_getAperture(self.native())
    }

    #[inline]
    pub unsafe fn get_shutter_speed(&self) -> f32 {
        bindgen::filament_Camera_getShutterSpeed(self.native())
    }

    #[inline]
    pub unsafe fn get_sensitivity(&self) -> f32 {
        bindgen::filament_Camera_getSensitivity(self.native())
    }

    #[inline]
    pub unsafe fn get_focal_length(&self) -> f64 {
        bindgen::filament_Camera_getFocalLength(self.native())
    }

    #[inline]
    pub unsafe fn set_focus_distance(&mut self, distance: f32) {
        bindgen::filament_Camera_setFocusDistance(self.native_mut(), distance)
    }

    #[inline]
    pub unsafe fn get_focus_distance(&self) -> f32 {
        bindgen::filament_Camera_getFocusDistance(self.native())
    }

    #[inline]
    pub unsafe fn inverse_projection(
        p: &bindgen::filament_math_mat4,
    ) -> bindgen::filament_math_mat4 {
        bindgen::filament_Camera_inverseProjection(p)
    }

    #[inline]
    pub unsafe fn inverse_projection_float(
        p: &bindgen::filament_math_mat4f,
    ) -> bindgen::filament_math_mat4f {
        bindgen::filament_Camera_inverseProjection1(p)
    }

    #[inline]
    pub unsafe fn compute_effective_focal_length(focal_length: f64, focus_distance: f64) -> f64 {
        bindgen::filament_Camera_computeEffectiveFocalLength(focal_length, focus_distance)
    }

    #[inline]
    pub unsafe fn compute_effective_fov(fov_in_degrees: f64, focus_distance: f64) -> f64 {
        bindgen::filament_Camera_computeEffectiveFov(fov_in_degrees, focus_distance)
    }
}
