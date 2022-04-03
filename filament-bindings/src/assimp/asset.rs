use core::fmt;
use std::{error::Error, ffi::CString, path::Path, ptr};

use crate::{
    asset::CameraInfo,
    backend::{BufferDescriptor, ElementType, PrimitiveType},
    filament::{
        self, sRGBColor, Aabb, Bounds, Engine, IndexBuffer, IndexBufferBuilder, Material,
        MaterialBuilder, MaterialInstance, RenderableBuilder, RgbaType, VertexAttribute,
        VertexBuffer, VertexBufferBuilder,
    },
    math::{Float3, Float4, Half4, Mat3f, Mat4f, Short4, Ushort2},
    utils,
};
use russimp_sys::aiScene;

use crate::asset::Asset;

use super::helper::{
    compute_aabb, convert_uv, count_vertices, get_min_max_uv, transmute_ai_vector,
    transmute_ai_vector_3d_arr,
};

const RESOURCES_AIDEFAULTMAT_DATA: &'static [u8] = include_bytes!("aiDefaultMat.filamat");
const RESOURCES_AIDEFAULTTRANS_DATA: &'static [u8] = include_bytes!("aiDefaultTrans.filamat");

pub struct AssimpAsset {
    renderables: Vec<utils::Entity>,
    materials: Vec<Material>,
    material_instances: Vec<MaterialInstance>,
    vertex_buffer: VertexBuffer,
    index_buffer: IndexBuffer,
    aabb: Aabb,
    root_entity: utils::Entity,
    main_camera: Option<CameraInfo>,
}

pub struct AssimpData<'a> {
    indices: &'a mut Vec<u32>,
    positions: &'a mut Vec<Half4>,
    tangents: &'a mut Vec<Short4>,
    tex_coords0: &'a mut Vec<Ushort2>,
    tex_coords1: &'a mut Vec<Ushort2>,
    snorm_uv0: bool,
    snorm_uv1: bool,
    meshes: &'a mut Vec<AssimpMesh>,
}

struct AssimpMesh {
    node_name: Vec<i8>,
    parent_index: Option<usize>,
    indices_offset: usize,
    indices_count: usize,
    parts: Vec<AssimpMeshPart>,
    aabb: filament::Aabb,
    acc_aabb: filament::Aabb,
    transform: Mat4f,
    acc_transform: Mat4f,
}

struct AssimpMeshPart {
    indices_offset: usize,
    indices_count: usize,
    opacity: f32,
    metallic: f32,
    roughness: f32,
    reflectance: f32,
    base_color: filament::sRGBColor,
}

impl AssimpAsset {
    pub fn from_memory(
        engine: &mut filament::Engine,
        buffer: &[u8],
        hint: &str,
    ) -> Result<Self, E> {
        unsafe {
            let c_hint = CString::new(hint).map_err(|_| E::InvalidString)?;
            let ai_scene = russimp_sys::aiImportFileFromMemory(
                buffer.as_ptr() as *const _,
                buffer.len() as _,
                (russimp_sys::aiPostProcessSteps_aiProcess_GenSmoothNormals
                    | russimp_sys::aiPostProcessSteps_aiProcess_CalcTangentSpace
                    | russimp_sys::aiPostProcessSteps_aiProcess_GenUVCoords
                    | russimp_sys::aiPostProcessSteps_aiProcess_FindInstances
                    | russimp_sys::aiPostProcessSteps_aiProcess_OptimizeMeshes
                    | russimp_sys::aiPostProcessSteps_aiProcess_JoinIdenticalVertices
                    | russimp_sys::aiPostProcessSteps_aiProcess_ImproveCacheLocality
                    | russimp_sys::aiPostProcessSteps_aiProcess_SortByPType
                    // | russimp_sys::aiPostProcessSteps_aiProcess_PreTransformVertices
                    | russimp_sys::aiPostProcessSteps_aiProcess_Triangulate) as u32,
                c_hint.as_ptr(),
            );
            if ai_scene.is_null() {
                return Err(E::FailedLoadModel);
            }

            let default_color_material = {
                let mut material_builder = MaterialBuilder::new().ok_or(E::InternalError)?;
                material_builder.package(RESOURCES_AIDEFAULTMAT_DATA);
                material_builder.build(engine).ok_or(E::InternalError)?
            };
            let default_transparent_color_material = {
                let mut material_builder = MaterialBuilder::new().ok_or(E::InternalError)?;
                material_builder.package(RESOURCES_AIDEFAULTTRANS_DATA);
                material_builder.build(engine).ok_or(E::InternalError)?
            };
            let asset = Self::new(
                engine,
                &ai_scene.read(),
                default_color_material,
                default_transparent_color_material,
            )?;

            russimp_sys::aiReleaseImport(ai_scene);

            Ok(asset)
        }
    }
    pub fn from_file(engine: &mut filament::Engine, filename: impl AsRef<Path>) -> Result<Self, E> {
        unsafe {
            let c_filename = filename
                .as_ref()
                .to_str()
                .and_then(|f| CString::new(f).ok())
                .ok_or(E::InvalidString)?;
            let ai_scene = russimp_sys::aiImportFile(
                c_filename.as_ptr(),
                (russimp_sys::aiPostProcessSteps_aiProcess_GenSmoothNormals
                    | russimp_sys::aiPostProcessSteps_aiProcess_CalcTangentSpace
                    | russimp_sys::aiPostProcessSteps_aiProcess_GenUVCoords
                    | russimp_sys::aiPostProcessSteps_aiProcess_FindInstances
                    | russimp_sys::aiPostProcessSteps_aiProcess_OptimizeMeshes
                    | russimp_sys::aiPostProcessSteps_aiProcess_JoinIdenticalVertices
                    | russimp_sys::aiPostProcessSteps_aiProcess_ImproveCacheLocality
                    | russimp_sys::aiPostProcessSteps_aiProcess_SortByPType
                    // | russimp_sys::aiPostProcessSteps_aiProcess_PreTransformVertices
                    | russimp_sys::aiPostProcessSteps_aiProcess_Triangulate) as u32,
            );
            if ai_scene.is_null() {
                return Err(E::FailedLoadModel);
            }

            let default_color_material = {
                let mut material_builder = MaterialBuilder::new().ok_or(E::InternalError)?;
                material_builder.package(RESOURCES_AIDEFAULTMAT_DATA);
                material_builder.build(engine).ok_or(E::InternalError)?
            };
            let default_transparent_color_material = {
                let mut material_builder = MaterialBuilder::new().ok_or(E::InternalError)?;
                material_builder.package(RESOURCES_AIDEFAULTTRANS_DATA);
                material_builder.build(engine).ok_or(E::InternalError)?
            };
            let asset = Self::new(
                engine,
                &ai_scene.read(),
                default_color_material,
                default_transparent_color_material,
            )?;

            russimp_sys::aiReleaseImport(ai_scene);

            Ok(asset)
        }
    }
    pub(crate) fn new(
        engine: &mut filament::Engine,
        scene: &aiScene,
        default_color_material: Material,
        default_transparent_color_material: Material,
    ) -> Result<Self, E> {
        unsafe {
            let root_node = *scene.mRootNode;
            let (total_vertex_count, total_index_count) = count_vertices(&root_node, &scene);

            let mut positions = Vec::with_capacity(total_vertex_count);
            let mut tangents = Vec::with_capacity(total_vertex_count);
            let mut tex_coords0 = Vec::with_capacity(total_vertex_count);
            let mut tex_coords1 = Vec::with_capacity(total_vertex_count);
            let mut indices = Vec::with_capacity(total_index_count);

            let (min_uv0, max_uv0) = get_min_max_uv(&scene, &root_node, 0);
            let (min_uv1, max_uv1) = get_min_max_uv(&scene, &root_node, 1);

            let snorm_uv0 = min_uv0.vec[0] >= -1.0
                && min_uv0.vec[0] <= 1.0
                && max_uv0.vec[0] >= -1.0
                && max_uv0.vec[0] <= 1.0
                && min_uv0.vec[1] >= -1.0
                && min_uv0.vec[1] <= 1.0
                && max_uv0.vec[1] >= -1.0
                && max_uv0.vec[1] <= 1.0;

            let snorm_uv1 = min_uv1.vec[0] >= -1.0
                && min_uv1.vec[0] <= 1.0
                && max_uv1.vec[0] >= -1.0
                && max_uv1.vec[0] <= 1.0
                && min_uv1.vec[1] >= -1.0
                && min_uv1.vec[1] <= 1.0
                && max_uv1.vec[1] >= -1.0
                && max_uv1.vec[1] <= 1.0;

            let mut meshes: Vec<AssimpMesh> = Vec::new();

            let mut data = AssimpData {
                meshes: &mut meshes,
                indices: &mut indices,
                positions: &mut positions,
                snorm_uv0,
                snorm_uv1,
                tangents: &mut tangents,
                tex_coords0: &mut tex_coords0,
                tex_coords1: &mut tex_coords1,
            };

            process_node(&mut data, &scene, &root_node, None);

            let mut aabb = filament::Aabb {
                min: Float3::new(f32::MAX, f32::MAX, f32::MAX),
                max: Float3::new(f32::MIN, f32::MIN, f32::MIN),
            };

            // find bounding box of entire model
            for mesh in meshes.iter() {
                let aabb_min = mesh.acc_aabb.min;
                let aabb_max = mesh.acc_aabb.max;

                for i in 0..3 {
                    if !aabb_min[i].is_infinite() && !aabb_max[i].is_infinite() {
                        if aabb.min[i] > aabb.max[i] {
                            aabb.min[i] = aabb_min[i];
                            aabb.max[i] = aabb_max[i];
                        } else {
                            aabb.min[i] = aabb_min[i].min(aabb.min[i]);
                            aabb.max[i] = aabb_max[i].max(aabb.max[i]);
                        }
                    }
                }
            }

            let mut vertex_buffer = {
                let mut vertex_buffer_builder =
                    VertexBufferBuilder::new().ok_or(E::InternalError)?;

                vertex_buffer_builder
                    .vertex_count(positions.len() as u32)
                    .buffer_count(4)
                    .attribute(VertexAttribute::POSITION, 0, ElementType::HALF4, 0, 0)
                    .attribute(VertexAttribute::TANGENTS, 1, ElementType::SHORT4, 0, 0)
                    .normalized(VertexAttribute::TANGENTS, true);

                if snorm_uv0 {
                    vertex_buffer_builder
                        .attribute(VertexAttribute::UV0, 2, ElementType::SHORT2, 0, 0)
                        .normalized(VertexAttribute::UV0, false);
                } else {
                    vertex_buffer_builder.attribute(
                        VertexAttribute::UV0,
                        2,
                        ElementType::HALF2,
                        0,
                        0,
                    );
                }

                if snorm_uv1 {
                    vertex_buffer_builder
                        .attribute(VertexAttribute::UV1, 3, ElementType::SHORT2, 0, 0)
                        .normalized(VertexAttribute::UV1, true);
                } else {
                    vertex_buffer_builder.attribute(
                        VertexAttribute::UV1,
                        3,
                        ElementType::HALF2,
                        0,
                        0,
                    );
                }

                vertex_buffer_builder
                    .build(engine)
                    .ok_or(E::InternalError)?
            };

            vertex_buffer
                .set_buffer_at(engine, 0, BufferDescriptor::new(positions), 0)
                .set_buffer_at(engine, 1, BufferDescriptor::new(tangents), 0)
                .set_buffer_at(engine, 2, BufferDescriptor::new(tex_coords0), 0)
                .set_buffer_at(engine, 3, BufferDescriptor::new(tex_coords1), 0);

            let mut index_buffer = IndexBufferBuilder::new()
                .ok_or(E::InternalError)?
                .index_count(indices.len() as u32)
                .build(engine)
                .ok_or(E::InternalError)?;

            index_buffer.set_buffer(engine, BufferDescriptor::new(indices), 0);

            let mut entity_manager = engine.get_entity_manager().ok_or(E::InternalError)?;
            let mut renderables = Vec::with_capacity(meshes.len());
            for _ in 0..meshes.len() {
                renderables.push(entity_manager.create());
            }

            let root_entity = entity_manager.create();

            let mut transform_manager = engine.get_transform_manager().ok_or(E::InternalError)?;
            transform_manager.create_with_parent_transform_float(
                &root_entity,
                None,
                &Mat4f::default(),
            );

            let mut material_instances: Vec<MaterialInstance> = Vec::with_capacity(meshes.len());

            for (mesh_index, mesh) in meshes.iter().enumerate() {
                let mut builder =
                    RenderableBuilder::new(mesh.parts.len()).ok_or(E::InternalError)?;
                builder
                    .bounding_box(&Bounds {
                        center: mesh.aabb.center(),
                        half_extent: mesh.aabb.extent(),
                    })
                    .screen_space_contact_shadows(true);

                for (part_index, part) in mesh.parts.iter().enumerate() {
                    builder.geometry_offset(
                        part_index,
                        PrimitiveType::TRIANGLES,
                        &mut vertex_buffer,
                        &mut index_buffer,
                        part.indices_offset,
                        part.indices_count,
                    );

                    let mut color_material;
                    if part.opacity < 1.0 {
                        color_material = default_transparent_color_material
                            .create_instance()
                            .ok_or(E::InternalError)?;
                        color_material
                            .set_rgba_parameter(
                                "baseColor",
                                RgbaType::sRGB,
                                Float4::new(
                                    part.base_color.0[0],
                                    part.base_color.0[1],
                                    part.base_color.0[2],
                                    part.opacity,
                                ),
                            )
                            .ok()
                            .ok_or(E::InvalidString)?;
                    } else {
                        color_material = default_color_material
                            .create_instance()
                            .ok_or(E::InternalError)?;
                        color_material
                            .set_rgb_parameter(
                                "baseColor",
                                filament::RgbType::sRGB,
                                part.base_color.0,
                            )
                            .ok()
                            .ok_or(E::InvalidString)?;
                        color_material
                            .set_float_parameter("reflectance", &part.reflectance)
                            .ok()
                            .ok_or(E::InvalidString)?;
                    }
                    color_material
                        .set_float_parameter("metallic", &part.metallic)
                        .ok()
                        .ok_or(E::InvalidString)?;
                    color_material
                        .set_float_parameter("roughness", &part.roughness)
                        .ok()
                        .ok_or(E::InvalidString)?;
                    builder.material(part_index, &mut color_material);

                    material_instances.push(color_material);
                }
                let entity = renderables[mesh_index];
                if !mesh.parts.is_empty() {
                    builder.build(engine, &entity);
                }

                let parent_tansform_instance = if let Some(parent_index) = mesh.parent_index {
                    transform_manager
                        .get_instance(&renderables[parent_index])
                        .ok_or(E::InternalError)?
                } else {
                    transform_manager
                        .get_instance(&root_entity)
                        .ok_or(E::InternalError)?
                };
                transform_manager.create_with_parent_transform_float(
                    &entity,
                    Some(&parent_tansform_instance),
                    &mesh.transform,
                );
            }

            let main_camera = if (*scene).mNumCameras > 0 {
                let ai_camera = **(*scene).mCameras;
                let camera_name = &ai_camera.mName.data[0..ai_camera.mName.length as usize];
                if let Some(node) = meshes.iter().find(|m| m.node_name == camera_name) {
                    Some(CameraInfo {
                        aspect: ai_camera.mAspect as f64,
                        horizontal_fov: (ai_camera.mHorizontalFOV / std::f32::consts::PI * 180.0)
                            as f64,
                        look_at: transmute_ai_vector(ai_camera.mLookAt),
                        orthographic_width: ai_camera.mOrthographicWidth as f64,
                        position: (node.acc_transform
                            * Float4::from_vec3(transmute_ai_vector(ai_camera.mPosition), 1.0))
                        .xyz(),
                        up: transmute_ai_vector(ai_camera.mUp),
                    })
                } else {
                    dbg!("Can't find camera node");
                    None
                }
            } else {
                None
            };

            Ok(AssimpAsset {
                renderables,
                materials: vec![default_color_material, default_transparent_color_material],
                material_instances,
                vertex_buffer,
                index_buffer,
                root_entity,
                aabb,
                main_camera,
            })
        }
    }
}

impl Asset for AssimpAsset {
    fn get_renderables(&self) -> &[utils::Entity] {
        self.renderables.as_slice()
    }

    fn get_root_entity(&self) -> &utils::Entity {
        &self.root_entity
    }

    fn get_aabb(&self) -> &filament::Aabb {
        &self.aabb
    }

    fn get_main_camera(&self) -> Option<&CameraInfo> {
        self.main_camera.as_ref()
    }

    fn destory(&mut self, engine: &mut Engine) {
        unsafe {
            engine.destroy_vertex_buffer(&mut self.vertex_buffer);
            engine.destroy_index_buffer(&mut self.index_buffer);

            for material_instance in self.material_instances.iter_mut() {
                engine.destroy_material_instance(material_instance);
            }

            for material in self.materials.iter_mut() {
                engine.destroy_material(material);
            }

            let mut entity_manager = engine.get_entity_manager().unwrap();
            for renderable in self.renderables.iter_mut() {
                engine.destroy_entity_components(&renderable);
                entity_manager.destory(renderable);
            }

            engine.destroy_entity_components(&self.root_entity);
            entity_manager.destory(&mut self.root_entity);
        }
    }
}

unsafe fn process_node(
    data: &mut AssimpData,
    scene: &aiScene,
    node: &russimp_sys::aiNode,
    parent_index: Option<usize>,
) {
    let current_transform = {
        let m44 = node.mTransformation;
        Mat4f([
            m44.a1, m44.b1, m44.c1, m44.d1, m44.a2, m44.b2, m44.c2, m44.d2, m44.a3, m44.b3, m44.c3,
            m44.d3, m44.a4, m44.b4, m44.c4, m44.d4,
        ])
    };
    let node_name = node.mName.data[0..node.mName.length as usize].to_vec();
    let mut current_mesh = AssimpMesh {
        node_name,
        transform: current_transform,
        parent_index,
        indices_offset: data.positions.len(),
        indices_count: 0,
        acc_transform: if let Some(parent_index) = parent_index {
            data.meshes[parent_index as usize].acc_transform
        } else {
            Mat4f::default()
        } * current_transform,
        parts: Default::default(),
        aabb: Default::default(),
        acc_aabb: Default::default(),
    };

    let mut total_indices = 0usize;

    for i in 0..node.mNumMeshes {
        let mesh_index = node.mMeshes.add(i as usize).read();
        let mesh = scene.mMeshes.add(mesh_index as usize).read().read();

        let positions = transmute_ai_vector_3d_arr(mesh.mVertices, mesh.mNumVertices as usize);
        let tangents = transmute_ai_vector_3d_arr(mesh.mTangents, mesh.mNumVertices as usize);
        let bitangents = transmute_ai_vector_3d_arr(mesh.mBitangents, mesh.mNumVertices as usize);
        let normals = transmute_ai_vector_3d_arr(mesh.mNormals, mesh.mNumVertices as usize);
        let tex_coords0 =
            transmute_ai_vector_3d_arr(mesh.mTextureCoords[0], mesh.mNumVertices as usize);
        let tex_coords1 =
            transmute_ai_vector_3d_arr(mesh.mTextureCoords[1], mesh.mNumVertices as usize);

        if mesh.mNumVertices > 0 && mesh.mNumFaces > 0 {
            let faces = core::slice::from_raw_parts(mesh.mFaces, mesh.mNumFaces as usize);
            let indices_offset = data.positions.len();

            for j in 0..positions.len() {
                let normal = normals[j];
                let tangent;
                let bitangent;

                // Assimp always returns 3D tex coords but we only support 2D tex coords.
                let tex_coord0 = if !tex_coords0.is_empty() {
                    tex_coords0[j].xy()
                } else {
                    Default::default()
                };
                let tex_coord1 = if !tex_coords1.is_empty() {
                    tex_coords1[j].xy()
                } else {
                    Default::default()
                };

                // If the tangent and bitangent don't exist, make arbitrary ones. This only
                // occurs when the mesh is missing texture coordinates, because assimp
                // computes tangents for us. (search up for aiProcess_CalcTangentSpace)
                if tangents.is_empty() {
                    bitangent = Float3::cross(&normal, &Float3::new(1.0, 0.0, 0.0)).normalize();
                    tangent = Float3::cross(&bitangent, &normal).normalize();
                } else {
                    tangent = tangents[j];
                    bitangent = bitangents[j];
                }

                let q = Mat3f::from((tangent, bitangent, normal)).pack_tangent_frame();
                data.tangents.push(Float4::from(q).pack_snorm16());
                data.tex_coords0
                    .push(convert_uv(&tex_coord0, data.snorm_uv0));
                data.tex_coords1
                    .push(convert_uv(&tex_coord1, data.snorm_uv1));

                data.positions.push(Half4::new(
                    half::f16::from_f32(positions[j][0]),
                    half::f16::from_f32(positions[j][1]),
                    half::f16::from_f32(positions[j][2]),
                    half::f16::from_f32(1.0),
                ));
            }

            let indices_count = faces.len() * faces[0].mNumIndices as usize;
            let index_buffer_offset = data.indices.len();
            total_indices += indices_count;

            for face in faces {
                for k in 0..face.mNumIndices {
                    data.indices
                        .push((face.mIndices.add(k as usize).read() + indices_offset as u32) as u32)
                }
            }

            let material_id = mesh.mMaterialIndex;
            let material = scene.mMaterials.add(material_id as usize).read().read();

            let material_diffuse_key = CString::new("$clr.diffuse").unwrap();
            let material_opacity_key = CString::new("$mat.opacity").unwrap();
            let material_shininess_key = CString::new("$mat.shininess").unwrap();
            let material_color_specular_key = CString::new("$clr.specular").unwrap();

            let mut color = russimp_sys::aiColor4D {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            };
            let mut base_color = if russimp_sys::aiGetMaterialColor(
                &material,
                material_diffuse_key.as_ptr(),
                0,
                0,
                &mut color,
            ) == russimp_sys::aiReturn_aiReturn_SUCCESS
            {
                sRGBColor([color.r, color.g, color.b].into())
            } else {
                sRGBColor(1.0.into())
            };

            let mut opacity = 1.0;
            if russimp_sys::aiGetMaterialFloatArray(
                &material,
                material_opacity_key.as_ptr(),
                0,
                0,
                &mut opacity,
                ptr::null_mut(),
            ) != russimp_sys::aiReturn_aiReturn_SUCCESS
            {
                opacity = 1.0f32;
            }
            if opacity <= 0.0 {
                opacity = 1.0;
            }

            let mut shininess = 1.0;
            if russimp_sys::aiGetMaterialFloatArray(
                &material,
                material_shininess_key.as_ptr(),
                0,
                0,
                &mut shininess,
                ptr::null_mut(),
            ) != russimp_sys::aiReturn_aiReturn_SUCCESS
            {
                shininess = 0.0;
            }

            // convert shininess to roughness
            let roughness = (2.0 / (shininess + 2.0)).sqrt();

            let mut metallic = 0.0f32;
            let mut reflectance = 0.5f32;
            if russimp_sys::aiGetMaterialColor(
                &material,
                material_color_specular_key.as_ptr(),
                0,
                0,
                &mut color,
            ) == russimp_sys::aiReturn_aiReturn_SUCCESS
            {
                // if there's a non-grey specular color, assume a metallic surface
                if color.r != color.g && color.r != color.b {
                    metallic = 1.0;
                    base_color = sRGBColor([color.r, color.g, color.b].into());
                } else {
                    if base_color == sRGBColor(0.0.into()) {
                        metallic = 1.0;
                        base_color = sRGBColor([color.r, color.g, color.b].into());
                    } else {
                        // the conversion formula is correct?
                        reflectance = (color.r / 0.16).sqrt();
                    }
                }
            }

            current_mesh.parts.push(AssimpMeshPart {
                indices_offset: index_buffer_offset,
                indices_count,
                base_color,
                opacity,
                roughness,
                metallic,
                reflectance,
            });
        }
    }

    current_mesh.indices_count = total_indices;

    if current_mesh.indices_count == 0 {
        current_mesh.aabb = filament::Aabb {
            min: Float3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Float3::new(f32::MIN, f32::MIN, f32::MIN),
        };

        current_mesh.acc_aabb = filament::Aabb {
            min: Float3::new(f32::MAX, f32::MAX, f32::MAX),
            max: Float3::new(f32::MIN, f32::MIN, f32::MIN),
        };
    } else {
        current_mesh.aabb = compute_aabb(
            data.positions.as_slice(),
            &data.indices[current_mesh.indices_offset
                ..current_mesh.indices_offset + current_mesh.indices_count],
        );

        current_mesh.acc_aabb = current_mesh.aabb.transform(current_mesh.acc_transform);
    }

    data.meshes.push(current_mesh);

    if node.mNumChildren > 0 {
        let parent_index = Some(data.meshes.len() - 1);
        for i in 0..node.mNumChildren {
            process_node(
                data,
                scene,
                &node.mChildren.add(i as usize).read().read(),
                parent_index,
            );
        }
    }
}

#[derive(Debug, Clone)]
pub enum AssimpAssetError {
    FailedLoadModel,
    InvalidString,
    InternalError,
}

impl Error for AssimpAssetError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for AssimpAssetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::FailedLoadModel => write!(f, "Failed load model."),
            &Self::InvalidString => write!(f, "Invalid string."),
            &Self::InternalError => write!(f, "Internal error."),
        }
    }
}

type E = AssimpAssetError;
