use crate::{
    bindgen,
    filament::{Engine, IndexBuffer, MaterialInstance, VertexBuffer},
    utils::Entity,
};

pub struct MeshReader {}

pub struct Mesh {
    pub renderable: Entity,
    pub vertex_buffer: Option<VertexBuffer>,
    pub index_buffer: Option<IndexBuffer>,
}

impl Mesh {
    pub(crate) fn try_from_native(native: bindgen::filamesh_MeshReader_Mesh) -> Option<Self> {
        Some(Self {
            renderable: Entity::try_from_native(native.renderable)?,
            index_buffer: IndexBuffer::try_from_native(native.indexBuffer),
            vertex_buffer: VertexBuffer::try_from_native(native.vertexBuffer),
        })
    }
}

impl MeshReader {
    pub unsafe fn load_mesh_from_buffer_default_material(
        engine: &mut Engine,
        data: &'static [u8],
        default_material: &mut MaterialInstance,
    ) -> Option<Mesh> {
        let native_mesh = bindgen::filamesh_MeshReader_loadMeshFromBuffer1(
            engine.native_mut(),
            data.as_ptr() as *const _,
            None,
            std::ptr::null_mut(),
            default_material.native_mut(),
        );
        println!("{:?}", native_mesh);
        std::mem::forget(data);
        Mesh::try_from_native(native_mesh)
    }
}
