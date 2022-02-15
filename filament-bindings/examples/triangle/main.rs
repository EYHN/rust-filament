use std::{ffi::CString, fs::File, io::Write, os, ptr::null_mut};

use filament_bindings::{
    filament_Camera_Projection_ORTHO, filament_Camera_setProjection, filament_Engine,
    filament_Engine_createCamera, filament_Engine_createRenderer, filament_Engine_createScene,
    filament_Engine_createSwapChain1, filament_Engine_createView, filament_Engine_destroy,
    filament_Engine_flushAndWait, filament_Engine_getEntityManager, filament_IndexBuffer_Builder,
    filament_IndexBuffer_IndexType_USHORT, filament_IndexBuffer_setBuffer,
    filament_MaterialInstance_setParameter, filament_Material_Builder,
    filament_Material_getDefaultInstance, filament_RenderableManager_Builder,
    filament_Renderer_ClearOptions, filament_Renderer_beginFrame, filament_Renderer_endFrame,
    filament_Renderer_readPixels, filament_Renderer_render, filament_Renderer_setClearOptions,
    filament_Scene_addEntity, filament_TextureSampler, filament_Texture_Builder,
    filament_Texture_setImage, filament_VertexAttribute_POSITION, filament_VertexBuffer_Builder,
    filament_VertexBuffer_setBufferAt, filament_View_setCamera, filament_View_setScene,
    filament_View_setViewport, filament_Viewport, filament_backend_Backend_OPENGL,
    filament_backend_BufferDescriptor, filament_backend_BufferDescriptor_Callback,
    filament_backend_ElementType_FLOAT2, filament_backend_PixelBufferDescriptor,
    filament_backend_PixelBufferDescriptor_PixelDataFormat,
    filament_backend_PixelBufferDescriptor_PixelDataType,
    filament_backend_PixelBufferDescriptor__bindgen_ty_1,
    filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1,
    filament_backend_PixelDataFormat_RGB, filament_backend_PixelDataFormat_RGBA,
    filament_backend_PixelDataType_UBYTE, filament_backend_PrimitiveType_TRIANGLES,
    filament_backend_SamplerCompareFunc_LE, filament_backend_SamplerCompareMode_NONE,
    filament_backend_SamplerMagFilter_NEAREST, filament_backend_SamplerParams,
    filament_backend_SamplerParams__bindgen_ty_1,
    filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1,
    filament_backend_SamplerWrapMode_CLAMP_TO_EDGE, filament_backend_TextureFormat_RGB8,
    filament_backend_Viewport, filament_math_float4, utils_Entity, utils_EntityManager_create,
};

const MATERIAL_BYTES: &'static [u8] = include_bytes!("texture_unlit_ogl.filamat");

#[repr(C)]
#[derive(Clone, Default)]
struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
struct Vertex {
    pub position: [f32; 2],
    pub uv: [f32; 2],
}

fn triangle_data() -> (Vec<Vertex>, Vec<u16>, Vec<RgbColor>) {
    let mut texture_data = vec![RgbColor::default(); 256 * 256];
    for y in 0..256 {
        for x in 0..256 {
            texture_data[y * 256 + x] = RgbColor {
                r: x as u8,
                g: y as u8,
                b: 0,
            };
        }
    }

    (
        vec![
            Vertex {
                position: [1.0, 0.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.5, 1.0],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.0],
                uv: [0.0, 0.0],
            },
        ],
        vec![0, 1, 2],
        texture_data,
    )
}

fn main() {
    unsafe {
        let mut engine =
            filament_Engine::create(filament_backend_Backend_OPENGL, null_mut(), null_mut());
        let swap_chain = filament_Engine_createSwapChain1(engine, 800, 600, 0);
        let renderer = filament_Engine_createRenderer(engine);
        let view = filament_Engine_createView(engine);
        let scene = filament_Engine_createScene(engine);

        let entity_manager = filament_Engine_getEntityManager(engine);
        let mut camera_entity = utils_Entity::default();
        utils_EntityManager_create(entity_manager, 1, &mut camera_entity as *mut _);

        let camera = filament_Engine_createCamera(engine, camera_entity);
        let aspect = 800.0 / 600.0;
        let zoom = 1.0;

        filament_Camera_setProjection(
            camera,
            filament_Camera_Projection_ORTHO,
            -aspect * zoom,
            aspect * zoom,
            -zoom,
            zoom,
            0.0,
            10.0,
        );

        let viewport = filament_Viewport {
            _base: filament_backend_Viewport {
                left: 0,
                bottom: 0,
                width: 800,
                height: 600,
            },
        };

        filament_View_setViewport(view, &viewport as *const _);
        filament_View_setScene(view, scene);
        filament_View_setCamera(view, camera);

        let clear_options = filament_Renderer_ClearOptions {
            clearColor: filament_math_float4 {
                inner: [0.0, 0.0, 1.0, 1.0],
            },
            clear: true,
            discard: true,
        };
        filament_Renderer_setClearOptions(renderer, &clear_options as *const _);

        let (vertices, indices, texture_data) = triangle_data();

        let mut vertex_buffer_builder = filament_VertexBuffer_Builder::new();
        vertex_buffer_builder.vertexCount(3);
        vertex_buffer_builder.bufferCount(1);
        vertex_buffer_builder.attribute(
            filament_VertexAttribute_POSITION,
            0,
            filament_backend_ElementType_FLOAT2,
            0,
            16,
        );
        let vertex_buffer = vertex_buffer_builder.build(engine);
        let mut vertices_desc = make_buffer_descriptor(vertices, None);
        filament_VertexBuffer_setBufferAt(
            vertex_buffer,
            engine,
            0,
            &mut vertices_desc as *mut _,
            0,
        );

        let mut index_buffer_builder = filament_IndexBuffer_Builder::new();
        index_buffer_builder.indexCount(3);
        index_buffer_builder.bufferType(filament_IndexBuffer_IndexType_USHORT);
        let index_buffer = index_buffer_builder.build(engine);
        let mut indices_desc = make_buffer_descriptor(indices, None);
        filament_IndexBuffer_setBuffer(index_buffer, engine, &mut indices_desc as *mut _, 0);

        let mut sampler_params =
            filament_backend_SamplerParams__bindgen_ty_1__bindgen_ty_1::default();
        sampler_params.set_filterMag(filament_backend_SamplerMagFilter_NEAREST);
        sampler_params.set_filterMin(filament_backend_SamplerMagFilter_NEAREST);
        sampler_params.set_wrapS(filament_backend_SamplerWrapMode_CLAMP_TO_EDGE);
        sampler_params.set_wrapT(filament_backend_SamplerWrapMode_CLAMP_TO_EDGE);
        sampler_params.set_wrapR(filament_backend_SamplerWrapMode_CLAMP_TO_EDGE);
        sampler_params.set_anisotropyLog2(0);
        sampler_params.set_compareMode(filament_backend_SamplerCompareMode_NONE);
        sampler_params.set_compareFunc(filament_backend_SamplerCompareFunc_LE);

        let sampler = filament_TextureSampler {
            mSamplerParams: filament_backend_SamplerParams {
                __bindgen_anon_1: filament_backend_SamplerParams__bindgen_ty_1 {
                    __bindgen_anon_1: sampler_params,
                },
            },
        };

        let mut texture_builder = filament_Texture_Builder::new();
        texture_builder.width(256);
        texture_builder.height(256);
        texture_builder.format(filament_backend_TextureFormat_RGB8);
        let texture = texture_builder.build(engine);
        let mut texture_desc = make_pixel_buffer_descriptor(
            texture_data,
            filament_backend_PixelDataFormat_RGB,
            filament_backend_PixelDataType_UBYTE,
            None,
        );
        filament_Texture_setImage(texture, engine, 0, &mut texture_desc as *mut _);

        let mut material_builder = filament_Material_Builder::new();
        material_builder.package(
            MATERIAL_BYTES.as_ptr() as *const _,
            MATERIAL_BYTES.len() as u64,
        );
        let material = material_builder.build(engine);
        let material_instance = filament_Material_getDefaultInstance(material);
        let material_texture_name = CString::new("texture").unwrap();
        filament_MaterialInstance_setParameter(
            material_instance,
            material_texture_name.as_ptr(),
            texture,
            &sampler as *const _,
        );

        let mut renderable = utils_Entity::default();
        utils_EntityManager_create(entity_manager, 1, &mut renderable as *mut _);

        let mut renderable_manager_builder = filament_RenderableManager_Builder::new(1);
        renderable_manager_builder.culling(false);
        renderable_manager_builder.castShadows(false);
        renderable_manager_builder.receiveShadows(false);
        renderable_manager_builder.material(0, material_instance);
        renderable_manager_builder.geometry2(
            0,
            filament_backend_PrimitiveType_TRIANGLES,
            vertex_buffer,
            index_buffer,
        );
        let build_result = renderable_manager_builder.build(engine, renderable);
        println!("build result: {}", build_result);

        filament_Scene_addEntity(scene, renderable);

        filament_Renderer_beginFrame(renderer, swap_chain, 0);

        println!("start rendering");

        filament_Renderer_render(renderer, view);
        const BYTE_COUNT: usize = 800 * 600 * 4;
        let buffer = vec![0u8; BYTE_COUNT];
        let mut pixel = make_pixel_buffer_descriptor(
            buffer,
            filament_backend_PixelDataFormat_RGBA,
            filament_backend_PixelDataType_UBYTE,
            Some(pixelbuffer_read_callback),
        );
        filament_Renderer_readPixels(renderer, 0, 0, 800, 600, &mut pixel as *mut _);

        filament_Renderer_endFrame(renderer);

        filament_Engine_flushAndWait(engine);

        filament_Engine_destroy(&mut engine as *mut *mut filament_Engine);
    }
}

unsafe extern "C" fn pixelbuffer_read_callback(
    buffer: *mut os::raw::c_void,
    size: u64,
    _uesr: *mut os::raw::c_void,
) {
    let mut buffer: Vec<u8> = Vec::from_raw_parts(buffer as *mut _, size as usize, size as usize);
    convert_rgba_to_rgb(buffer.as_mut_ptr(), 800, 600);

    let slice_u8 = &buffer[..800 * 600 * 3];

    let mut file = File::create("triangle.ppm").unwrap();
    println!("The rendering result is written to triangle.ppm");
    file.write(format!("P6 {} {} {}\n", 800, 600, 255).as_bytes())
        .unwrap();
    file.write_all(slice_u8).unwrap();
    file.flush().unwrap();

    std::mem::drop(buffer);
}

unsafe fn convert_rgba_to_rgb(buffer: *mut u8, width: u32, height: u32) {
    let mut write_ptr = buffer;
    let mut read_ptr: *const u8 = buffer;

    let mut i: u32 = 0;
    let n = width * height;
    while i < n {
        write_ptr.write(*read_ptr);
        write_ptr.offset(1).write(*read_ptr.offset(1));
        write_ptr.offset(2).write(*read_ptr.offset(2));
        write_ptr = write_ptr.offset(3);
        read_ptr = read_ptr.offset(4);
        i += 1;
    }
}

fn make_buffer_descriptor<T: Sized>(
    mut data: Vec<T>,
    callback: filament_backend_BufferDescriptor_Callback,
) -> filament_backend_BufferDescriptor {
    let desc = filament_backend_BufferDescriptor {
        buffer: data.as_mut_ptr() as *mut _,
        size: (data.len() * std::mem::size_of::<T>()).try_into().unwrap(),
        mCallback: callback.or(Some(deallocate_rust_buffer)),
        mUser: null_mut(),
        mHandler: null_mut(),
    };
    std::mem::forget(data);
    desc
}

fn make_pixel_buffer_descriptor<T: Sized>(
    data: Vec<T>,
    format: filament_backend_PixelBufferDescriptor_PixelDataFormat,
    datatype: filament_backend_PixelBufferDescriptor_PixelDataType,
    callback: filament_backend_BufferDescriptor_Callback,
) -> filament_backend_PixelBufferDescriptor {
    filament_backend_PixelBufferDescriptor {
        _base: make_buffer_descriptor(data, callback),
        left: 0,
        top: 0,
        __bindgen_anon_1: filament_backend_PixelBufferDescriptor__bindgen_ty_1 {
            __bindgen_anon_1: filament_backend_PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1 {
                stride: 0,
                format: format,
            },
        },
        _bitfield_1: filament_backend_PixelBufferDescriptor::new_bitfield_1(datatype, 1),
        ..Default::default()
    }
}

pub unsafe extern "C" fn deallocate_rust_buffer(
    ptr: *mut std::ffi::c_void,
    size: u64,
    _user: *mut std::ffi::c_void,
) {
    let size = size as usize;
    std::mem::drop(Vec::from_raw_parts(ptr, size, size));
}
