use filament_bindings::{
    filament_Box, filament_RenderableManager_Builder, filament_RenderableManager_Instance,
    filament_RenderableManager_destroy, filament_RenderableManager_getInstance, size_t,
};

use crate::{backend::PrimitiveType, prelude::NativeHandle, utils::Entity};

use super::{IndexBuffer, MaterialInstance, VertexBuffer};

pub struct RenderableOptions {
    native: filament_RenderableManager_Builder,
    vertics: Vec<Option<VertexBuffer>>,
    indices: Vec<Option<IndexBuffer>>,
    material_instances: Vec<Option<MaterialInstance>>,
}

impl RenderableOptions {
    #[inline]
    pub fn new(count: usize) -> Self {
        Self {
            native: unsafe { filament_RenderableManager_Builder::new(count as size_t) },
            vertics: vec![None; count],
            indices: vec![None; count],
            material_instances: vec![None; count],
        }
    }

    #[inline]
    pub fn from(r: &mut RenderableOptions) -> Self {
        Self {
            native: unsafe { filament_RenderableManager_Builder::new1(&mut r.native) },
            vertics: r.vertics.clone(),
            indices: r.indices.clone(),
            material_instances: r.material_instances.clone(),
        }
    }

    #[inline]
    pub fn native_mut(&mut self) -> &mut filament_RenderableManager_Builder {
        &mut self.native
    }

    #[inline]
    pub fn geometry(
        &mut self,
        index: usize,
        primitive_type: PrimitiveType,
        vertices: &mut VertexBuffer,
        indices: &mut IndexBuffer,
    ) -> &mut Self {
        self.vertics[index] = Some(vertices.clone());
        self.indices[index] = Some(indices.clone());
        unsafe {
            self.native.geometry2(
                index as size_t,
                primitive_type.into(),
                vertices.native_mut(),
                indices.native_mut(),
            );
        };
        self
    }

    #[inline]
    pub fn geometry_range(
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
        self.vertics[index] = Some(vertices.clone());
        self.indices[index] = Some(indices.clone());
        unsafe {
            self.native.geometry(
                index as size_t,
                primitive_type.into(),
                vertices.native_mut(),
                indices.native_mut(),
                indices_offset as size_t,
                indices_min_index as size_t,
                indices_max_index as size_t,
                indices_const as size_t,
            );
        };
        self
    }

    #[inline]
    pub fn material(
        &mut self,
        index: size_t,
        material_instance: &mut MaterialInstance,
    ) -> &mut Self {
        self.material_instances[index as usize] = Some(material_instance.clone());
        unsafe {
            self.native.material(index, material_instance.native_mut());
        };
        self
    }

    #[inline]
    pub fn bounding_box(&mut self, axis_aligned_bounding_box: &filament_Box) -> &mut Self {
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
    pub fn morphing(&mut self, target_count: size_t) -> &mut Self {
        unsafe { self.native.morphing(target_count) };
        self
    }

    #[inline]
    pub fn blend_order(&mut self, primitive_index: size_t, order: u16) -> &mut Self {
        unsafe { self.native.blendOrder(primitive_index, order) };
        self
    }
}

impl Drop for RenderableOptions {
    #[inline]
    fn drop(&mut self) {
        unsafe { self.native.destruct() }
    }
}

pub struct Renderable {
    native: filament_RenderableManager_Instance,
    vertics: Vec<Option<VertexBuffer>>,
    indices: Vec<Option<IndexBuffer>>,
    material_instances: Vec<Option<MaterialInstance>>,
}

impl Renderable {
    #[inline]
    pub(crate) fn try_from_native(
        native: filament_RenderableManager_Instance,
        vertics: Vec<Option<VertexBuffer>>,
        indices: Vec<Option<IndexBuffer>>,
        material_instances: Vec<Option<MaterialInstance>>,
    ) -> Option<Self> {
        if native == 0 {
            None
        } else {
            Some(Self {
                native,
                vertics,
                indices,
                material_instances,
            })
        }
    }

    pub(crate) fn from_options(entity: &mut Entity, options: &mut RenderableOptions) -> Option<Self> {
        let native_builder = options.native_mut();
        unsafe { native_builder.build(entity.engine.native_mut(), entity.native_owned()) };
        let native = unsafe {
            filament_RenderableManager_getInstance(
                entity.engine.get_renderable_manger(),
                entity.native_owned(),
            )
        };
        Self::try_from_native(
            native,
            options.vertics.clone(),
            options.indices.clone(),
            options.material_instances.clone(),
        )
    }

    #[inline]
    pub(crate) fn destroy_instance(entity: &mut Entity) {
        unsafe {
            filament_RenderableManager_destroy(
                entity.engine.get_renderable_manger(),
                entity.native_owned(),
            )
        }
    }

    #[inline]
    pub fn native_owned(&self) -> filament_RenderableManager_Instance {
        self.native
    }
}
