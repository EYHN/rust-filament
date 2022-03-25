use crate::{
    filament::Aabb,
    math::{Float2, Float3, Float4, Half2, Half4, Mat4f, Short2, Ushort2},
};

pub unsafe fn count_vertices(
    node: &russimp_sys::aiNode,
    scene: &russimp_sys::aiScene,
) -> (usize, usize) {
    let mut total_vertex_count: usize = 0;
    let mut total_index_count: usize = 0;
    for i in 0..node.mNumMeshes {
        let mesh_index = node.mMeshes.add(i as usize).read() as usize;
        let mesh = scene.mMeshes.add(mesh_index).read().read();
        total_vertex_count += mesh.mNumVertices as usize;

        let first_face = mesh.mFaces.read();
        let num_faces = mesh.mNumFaces;
        total_index_count += num_faces as usize * first_face.mNumIndices as usize;
    }

    for i in 0..node.mNumChildren {
        let (vertex_count, index_count) =
            count_vertices(&node.mChildren.add(i as usize).read().read(), scene);
        total_vertex_count += vertex_count;
        total_index_count += index_count;
    }

    (total_vertex_count, total_index_count)
}

pub unsafe fn get_min_max_uv(
    scene: &russimp_sys::aiScene,
    node: &russimp_sys::aiNode,
    uv_index: u8,
) -> (Float2, Float2) {
    let mut min_uv = Float2::new(f32::MAX, f32::MAX);
    let mut max_uv = Float2::new(f32::MIN, f32::MIN);
    for i in 0..node.mNumMeshes {
        let mesh_index = node.mMeshes.add(i as usize).read() as usize;
        let mesh = scene.mMeshes.add(mesh_index).read().read();
        if mesh.mTextureCoords[uv_index as usize].is_null() {
            continue;
        }
        if mesh.mNumVertices == 0 || mesh.mNumFaces == 0 {
            continue;
        }

        let uv_list = std::slice::from_raw_parts(
            mesh.mTextureCoords[uv_index as usize],
            mesh.mNumVertices as usize,
        );

        for uv in uv_list {
            min_uv = Float2::new(uv.x.min(min_uv.vec[0]), uv.y.min(min_uv.vec[1]));
            max_uv = Float2::new(uv.x.max(max_uv.vec[0]), uv.y.max(max_uv.vec[1]));
        }
    }

    for i in 0..node.mNumChildren {
        let (child_min, child_max) = get_min_max_uv(
            scene,
            &node.mChildren.add(i as usize).read().read(),
            uv_index,
        );

        min_uv = child_min.min(min_uv);
        max_uv = child_max.max(max_uv);
    }

    (min_uv, max_uv)
}

pub unsafe fn transmute_ai_vector_3d_arr(
    data: *mut russimp_sys::aiVector3D,
    size: usize,
) -> &'static [Float3] {
    if size == 0 || data.is_null() {
        &[]
    } else {
        core::mem::transmute::<&[russimp_sys::aiVector3D], &[Float3]>(core::slice::from_raw_parts(
            data, size,
        ))
    }
}

pub unsafe fn transmute_ai_vector(data: russimp_sys::aiVector3D) -> Float3 {
    core::mem::transmute::<russimp_sys::aiVector3D, Float3>(data)
}

pub unsafe fn convert_uv(uv: &Float2, snorm_uvs: bool) -> Ushort2 {
    if snorm_uvs {
        let uvshort = uv.pack_snorm16();
        return core::mem::transmute::<Short2, Ushort2>(uvshort);
    } else {
        let uvhalf = Half2::new(half::f16::from_f32(uv[0]), half::f16::from_f32(uv[1]));
        return core::mem::transmute::<Half2, Ushort2>(uvhalf);
    }
}

pub unsafe fn compute_aabb(position: &[Half4], indices: &[u32]) -> Aabb {
    let mut min = Float3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = Float3::new(f32::MIN, f32::MIN, f32::MIN);
    for index in indices {
        let pos = position[*index as usize];
        let v = Float3::new(pos[0].to_f32(), pos[1].to_f32(), pos[2].to_f32());
        min = min.min(v);
        max = max.max(v);
    }

    Aabb { max, min }
}

pub unsafe fn compute_transformed_aabb(
    position: &[Half4],
    indices: &[u32],
    transform: Mat4f,
) -> Aabb {
    let mut min = Float3::new(f32::MAX, f32::MAX, f32::MAX);
    let mut max = Float3::new(f32::MIN, f32::MIN, f32::MIN);
    for index in indices {
        let pos = position[*index as usize];
        let v =
            (transform * Float4::new(pos[0].to_f32(), pos[1].to_f32(), pos[2].to_f32(), 1.0)).xyz();
        min = min.min(v);
        max = max.max(v);
    }

    Aabb { max, min }
}
