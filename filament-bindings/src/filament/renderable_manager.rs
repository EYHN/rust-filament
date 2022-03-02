use std::ptr;

use crate::{backend::PrimitiveType, bindgen, utils::Entity};

use super::{Bounds, Engine, IndexBuffer, MaterialInstance, VertexBuffer};

pub struct RenderableBuilder {
    native: ptr::NonNull<bindgen::filament_RenderableManager_Builder>,
}

impl RenderableBuilder {
    #[inline]
    #[allow(dead_code)]
    pub(crate) unsafe fn native(&self) -> *const bindgen::filament_RenderableManager_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn native_mut(&mut self) -> *mut bindgen::filament_RenderableManager_Builder {
        self.native.as_ptr()
    }

    #[inline]
    pub(crate) unsafe fn try_from_native(
        native: *mut bindgen::filament_RenderableManager_Builder,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(RenderableBuilder { native: ptr })
    }

    #[inline]
    pub unsafe fn new(count: usize) -> Option<Self> {
        Self::try_from_native(bindgen::helper_filament_renderable_manager_builder_create(
            count,
        ))
    }

    #[inline]
    pub unsafe fn geometry(
        &mut self,
        index: usize,
        primitive_type: PrimitiveType,
        vertices: &mut VertexBuffer,
        indices: &mut IndexBuffer,
    ) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_geometry2(
            self.native_mut(),
            index,
            primitive_type.into(),
            vertices.native_mut(),
            indices.native_mut(),
        );
        self
    }

    #[inline]
    pub unsafe fn geometry_range(
        &mut self,
        index: usize,
        primitive_type: PrimitiveType,
        vertices: &mut VertexBuffer,
        indices: &mut IndexBuffer,
        indices_offset: usize,
        indices_min_index: usize,
        indices_max_index: usize,
        indices_const: usize,
    ) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_geometry(
            self.native_mut(),
            index,
            primitive_type.into(),
            vertices.native_mut(),
            indices.native_mut(),
            indices_offset,
            indices_min_index,
            indices_max_index,
            indices_const,
        );
        self
    }

    #[inline]
    pub unsafe fn material(
        &mut self,
        index: usize,
        material_instance: &mut MaterialInstance,
    ) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_material(
            self.native_mut(),
            index,
            material_instance.native_mut(),
        );

        self
    }

    #[inline]
    pub unsafe fn bounding_box(&mut self, axis_aligned_bounding_box: &Bounds) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_boundingBox(
            self.native_mut(),
            axis_aligned_bounding_box.native_ptr(),
        );
        self
    }

    #[inline]
    pub unsafe fn layer_mask(&mut self, select: u8, values: u8) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_layerMask(self.native_mut(), select, values);
        self
    }

    #[inline]
    pub unsafe fn priority(&mut self, priority: u8) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_priority(self.native_mut(), priority);

        self
    }

    #[inline]
    pub unsafe fn culling(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_culling(self.native_mut(), enable);
        self
    }

    #[inline]
    pub unsafe fn light_channel(&mut self, channel: u32, enable: bool) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_lightChannel(
            self.native_mut(),
            channel,
            enable,
        );
        self
    }

    #[inline]
    pub unsafe fn cast_shadows(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_castShadows(self.native_mut(), enable);
        self
    }

    #[inline]
    pub unsafe fn receive_shadows(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_receiveShadows(self.native_mut(), enable);
        self
    }

    #[inline]
    pub unsafe fn screen_space_contact_shadows(&mut self, enable: bool) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_screenSpaceContactShadows(
            self.native_mut(),
            enable,
        );
        self
    }

    // TODO: skinning

    #[inline]
    pub unsafe fn morphing(&mut self, target_count: usize) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_morphing(self.native_mut(), target_count);
        self
    }

    #[inline]
    pub unsafe fn blend_order(&mut self, primitive_index: usize, order: u16) -> &mut Self {
        bindgen::filament_RenderableManager_Builder_blendOrder(
            self.native_mut(),
            primitive_index,
            order,
        );
        self
    }

    #[inline]
    pub unsafe fn build(&mut self, engine: &mut Engine, entity: &Entity) -> Option<&mut Self> {
        if bindgen::filament_RenderableManager_Builder_build(
            self.native_mut(),
            engine.native_mut(),
            entity.native_owned(),
        ) == bindgen::filament_RenderableManager_Builder_Result_Success
        {
            Some(self)
        } else {
            None
        }
    }
}

impl Drop for RenderableBuilder {
    #[inline]
    fn drop(&mut self) {
        unsafe { bindgen::helper_filament_renderable_manager_builder_delete(self.native_mut()) }
    }
}

pub struct RenderableManager {
    native: ptr::NonNull<bindgen::filament_RenderableManager>,
}

impl RenderableManager {
    #[inline]
    pub unsafe fn native(&self) -> *const bindgen::filament_RenderableManager {
        self.native.as_ptr()
    }

    #[inline]
    pub unsafe fn native_mut(&mut self) -> *mut bindgen::filament_RenderableManager {
        self.native.as_ptr()
    }

    #[inline]
    pub unsafe fn try_from_native(
        native: *mut bindgen::filament_RenderableManager,
    ) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }
}
