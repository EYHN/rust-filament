use std::{ptr, rc::Rc};

use filament_bindings::{
    filament_Camera, filament_Camera_Fov_HORIZONTAL, filament_Camera_Fov_VERTICAL,
    filament_Camera_Projection_ORTHO, filament_Camera_Projection_PERSPECTIVE,
    filament_Camera_setCustomProjection, filament_Camera_setCustomProjection1,
    filament_Camera_setLensProjection, filament_Camera_setProjection,
    filament_Camera_setProjection1, filament_Camera_setScaling, filament_Engine_createCamera,
    filament_Engine_destroyCameraComponent, filament_math_double2, filament_math_mat4, filament_Camera_setShift,
};
use num_enum::IntoPrimitive;

use crate::{prelude::NativeHandle, utils::Entity};

use super::Engine;

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

pub struct Camera(Rc<ptr::NonNull<filament_Camera>>, Engine, Entity);

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
    pub(crate) fn try_from_native(
        engine: Engine,
        entity: Entity,
        native: *mut filament_Camera,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Camera(Rc::new(ptr), engine, entity))
    }

    #[inline]
    pub fn create(engine: &mut Engine, entity: &Entity) -> Option<Self> {
        unsafe {
            Self::try_from_native(
                engine.clone(),
                entity.clone(),
                filament_Engine_createCamera(engine.native_mut(), entity.native_owned()),
            )
        }
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
}

impl Drop for Camera {
    #[inline]
    fn drop(&mut self) {
        if let Some(_) = Rc::get_mut(&mut self.0) {
            unsafe {
                filament_Engine_destroyCameraComponent(self.1.native_mut(), self.2.native_owned())
            };
        }
    }
}
