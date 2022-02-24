use std::ptr;

use crate::{backend::PrimitiveType, bindgen, utils::Entity};

use super::{Engine, IndexBuffer, MaterialInstance, VertexBuffer};

pub struct RenderableBuilder {
    native: bindgen::filament_RenderableManager_Builder,
}

impl RenderableBuilder {
    #[inline]
    pub unsafe fn new(count: usize) -> Self {
        Self {
            native: bindgen::filament_RenderableManager_Builder::new(count),
        }
    }

    #[inline]
    pub unsafe fn from(r: &mut RenderableBuilder) -> Self {
        Self {
            native: bindgen::filament_RenderableManager_Builder::new1(&mut r.native),
        }
    }

    #[inline]
    pub fn native_mut(&mut self) -> &mut bindgen::filament_RenderableManager_Builder {
        &mut self.native
    }

    #[inline]
    pub unsafe fn geometry(
        &mut self,
        index: usize,
        primitive_type: PrimitiveType,
        vertices: &mut VertexBuffer,
        indices: &mut IndexBuffer,
    ) -> &mut Self {
        self.native.geometry2(
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
        self.native.geometry(
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
    pub fn material(
        &mut self,
        index: usize,
        material_instance: &mut MaterialInstance,
    ) -> &mut Self {
        unsafe {
            self.native.material(index, material_instance.native_mut());
        };
        self
    }

    #[inline]
    pub fn bounding_box(&mut self, axis_aligned_bounding_box: &bindgen::filament_Box) -> &mut Self {
        unsafe {
            self.native.boundingBox(axis_aligned_bounding_box);
        };
        self
    }

    #[inline]
    pub fn layer_mask(&mut self, select: u8, values: u8) -> &mut Self {
        unsafe {
            self.native.layerMask(select, values);
        };
        self
    }

    #[inline]
    pub fn priority(&mut self, priority: u8) -> &mut Self {
        unsafe {
            self.native.priority(priority);
        };
        self
    }

    #[inline]
    pub fn culling(&mut self, enable: bool) -> &mut Self {
        unsafe { self.native.culling(enable) };
        self
    }

    #[inline]
    pub fn light_channel(&mut self, channel: u32, enable: bool) -> &mut Self {
        unsafe { self.native.lightChannel(channel, enable) };
        self
    }

    #[inline]
    pub fn cast_shadows(&mut self, enable: bool) -> &mut Self {
        unsafe { self.native.castShadows(enable) };
        self
    }

    #[inline]
    pub fn receive_shadows(&mut self, enable: bool) -> &mut Self {
        unsafe { self.native.receiveShadows(enable) };
        self
    }

    #[inline]
    pub fn screen_space_contact_shadows(&mut self, enable: bool) -> &mut Self {
        unsafe { self.native.screenSpaceContactShadows(enable) };
        self
    }

    // TODO: skinning

    #[inline]
    pub fn morphing(&mut self, target_count: usize) -> &mut Self {
        unsafe { self.native.morphing(target_count) };
        self
    }

    #[inline]
    pub fn blend_order(&mut self, primitive_index: usize, order: u16) -> &mut Self {
        unsafe { self.native.blendOrder(primitive_index, order) };
        self
    }

    #[inline]
    pub unsafe fn build(&mut self, engine: &mut Engine, entity: &Entity) -> Option<&mut Self> {
        if self
            .native
            .build(engine.native_mut(), entity.native_owned())
            == bindgen::filament_RenderableManager_Builder_Result_Success
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
        unsafe { self.native.destruct() }
    }
}

pub struct RenderableManager {
    native: ptr::NonNull<bindgen::filament_RenderableManager>,
}

impl RenderableManager {
    #[inline]
    pub fn native(&self) -> *const bindgen::filament_RenderableManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn native_mut(&mut self) -> *mut bindgen::filament_RenderableManager {
        self.native.as_ptr()
    }

    #[inline]
    pub fn try_from_native(native: *mut bindgen::filament_RenderableManager) -> Option<Self> {
        let ptr = ptr::NonNull::new(native)?;
        Some(Self { native: ptr })
    }
}
