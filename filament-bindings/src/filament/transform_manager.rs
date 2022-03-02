use std::ptr;

use crate::{
    bindgen,
    math::{Mat4, Mat4f},
    utils::Entity,
};

pub struct TransformManager {
    native: ptr::NonNull<bindgen::filament_TransformManager>,
}

impl TransformManager {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_TransformManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_TransformManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_TransformManager) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[repr(transparent)]
pub struct TransformManagerInstance {
    native: bindgen::filament_TransformManager_Instance,
}

impl TransformManagerInstance {
    #[inline]
    pub fn native_ptr(&self) -> *const bindgen::filament_TransformManager_Instance {
        &self.native
    }

    #[inline]
    pub fn native_ptr_mut(&mut self) -> *mut bindgen::filament_TransformManager_Instance {
        &mut self.native
    }

    #[inline]
    pub(crate) fn native_owned(&self) -> bindgen::filament_TransformManager_Instance {
        self.native.clone()
    }
}

impl TransformManager {
    #[inline]
    pub unsafe fn has_component(&self, e: &Entity) -> bool {
        bindgen::filament_TransformManager_hasComponent(self.native(), e.native_owned())
    }

    #[inline]
    pub unsafe fn get_instance(&self, e: &Entity) -> TransformManagerInstance {
        TransformManagerInstance {
            native: bindgen::filament_TransformManager_getInstance(self.native(), e.native_owned()),
        }
    }

    #[inline]
    pub unsafe fn set_accurate_translations_enabled(&mut self, enable: bool) {
        bindgen::filament_TransformManager_setAccurateTranslationsEnabled(self.native_mut(), enable)
    }

    #[inline]
    pub unsafe fn is_accurate_translations_enabled(&self) -> bool {
        bindgen::filament_TransformManager_isAccurateTranslationsEnabled(self.native())
    }

    #[inline]
    pub unsafe fn create_parent_transform_float(
        &mut self,
        entity: &Entity,
        parent: &TransformManagerInstance,
        local_transform: &Mat4f,
    ) {
        bindgen::filament_TransformManager_create(
            self.native_mut(),
            entity.native_owned(),
            parent.native_owned(),
            local_transform.native_ptr(),
        )
    }

    #[inline]
    pub unsafe fn create_parent_transform(
        &mut self,
        entity: &Entity,
        parent: &TransformManagerInstance,
        local_transform: &Mat4,
    ) {
        bindgen::filament_TransformManager_create1(
            self.native_mut(),
            entity.native_owned(),
            parent.native_owned(),
            local_transform.native_ptr(),
        )
    }

    #[inline]
    pub unsafe fn create_parent(&mut self, entity: &Entity, parent: &TransformManagerInstance) {
        bindgen::filament_TransformManager_create2(
            self.native_mut(),
            entity.native_owned(),
            parent.native_owned(),
        )
    }

    #[inline]
    pub unsafe fn create(&mut self, entity: &Entity) {
        bindgen::filament_TransformManager_create2(
            self.native_mut(),
            entity.native_owned(),
            bindgen::filament_TransformManager_Instance::default(),
        )
    }

    #[inline]
    pub unsafe fn destroy(&mut self, e: &Entity) {
        bindgen::filament_TransformManager_destroy(self.native_mut(), e.native_owned())
    }

    #[inline]
    pub unsafe fn set_parent(
        &mut self,
        i: &TransformManagerInstance,
        new_parent: &TransformManagerInstance,
    ) {
        bindgen::filament_TransformManager_setParent(
            self.native_mut(),
            i.native_owned(),
            new_parent.native_owned(),
        )
    }

    #[inline]
    pub unsafe fn get_parent(&self, i: &TransformManagerInstance) -> Option<Entity> {
        Entity::try_from_native(bindgen::filament_TransformManager_getParent(
            self.native(),
            i.native_owned(),
        ))
    }

    #[inline]
    pub unsafe fn get_child_count(&self, i: &TransformManagerInstance) -> usize {
        bindgen::filament_TransformManager_getChildCount(self.native(), i.native_owned())
    }

    #[inline]
    pub unsafe fn get_children(&self, i: &TransformManagerInstance) -> Vec<Entity> {
        let count = self.get_child_count(i);
        let mut children = vec![Entity::dangling(); count];
        bindgen::filament_TransformManager_getChildren(
            self.native(),
            i.native_owned(),
            children.as_mut_ptr() as *mut _,
            count,
        );
        children
    }

    #[inline]
    pub unsafe fn set_transform_float(
        &mut self,
        ci: &TransformManagerInstance,
        local_transform: &Mat4f,
    ) {
        bindgen::filament_TransformManager_setTransform(
            self.native_mut(),
            ci.native_owned(),
            local_transform.native_ptr(),
        )
    }

    #[inline]
    pub unsafe fn set_transform(&mut self, ci: &TransformManagerInstance, local_transform: &Mat4) {
        bindgen::filament_TransformManager_setTransform1(
            self.native_mut(),
            ci.native_owned(),
            local_transform.native_ptr(),
        )
    }

    #[inline]
    pub unsafe fn get_transform(&self, ci: &TransformManagerInstance) -> Mat4f {
        Mat4f::from_native(
            bindgen::filament_TransformManager_getTransform(self.native(), ci.native_owned())
                .read(),
        )
    }

    #[inline]
    pub unsafe fn get_transform_accurate(&self, ci: &TransformManagerInstance) -> Mat4 {
        Mat4::from_native(bindgen::filament_TransformManager_getTransformAccurate(
            self.native(),
            ci.native_owned(),
        ))
    }

    #[inline]
    pub unsafe fn get_world_transform(&self, ci: &TransformManagerInstance) -> Mat4f {
        Mat4f::from_native(
            bindgen::filament_TransformManager_getWorldTransform(self.native(), ci.native_owned())
                .read(),
        )
    }

    #[inline]
    pub unsafe fn get_world_transform_accurate(&self, ci: &TransformManagerInstance) -> Mat4 {
        Mat4::from_native(
            bindgen::filament_TransformManager_getWorldTransformAccurate(
                self.native(),
                ci.native_owned(),
            ),
        )
    }

    #[inline]
    pub unsafe fn open_local_transform_transaction(&mut self) {
        bindgen::filament_TransformManager_openLocalTransformTransaction(self.native_mut())
    }

    #[inline]
    pub unsafe fn commit_local_transform_transaction(&mut self) {
        bindgen::filament_TransformManager_commitLocalTransformTransaction(self.native_mut())
    }
}
