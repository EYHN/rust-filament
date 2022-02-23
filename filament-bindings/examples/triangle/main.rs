use filament_bindings::root::{
    filament::{
        backend::{self, Backend, BufferDescriptor, SamplerParams},
        Camera_Projection, Engine, IndexBuffer_Builder, IndexBuffer_IndexType, Material_Builder,
        RenderableManager_Builder, Renderer_ClearOptions, TextureSampler, Texture_Builder,
        VertexAttribute, VertexBuffer_Builder, Viewport,
    },
    utils::Entity,
};

use std::{
    ffi::CString,
    fs::File,
    io::Write,
    ptr::{self, null_mut},
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
                position: [1.0, -1.0],
                uv: [1.0, 0.0],
            },
            Vertex {
                position: [0.0, 1.0],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [-1.0, -1.0],
                uv: [0.0, 0.0],
            },
        ],
        vec![0, 1, 2],
        texture_data,
    )
}

fn main() {
    unsafe {
        let engine = &mut *Engine::create(Backend::OPENGL, null_mut(), null_mut());
        let swap_chain = engine.createSwapChain1(800, 600, 0);
        let renderer = &mut *engine.createRenderer();
        let view = &mut *engine.createView();
        let scene = &mut *engine.createScene();

        let entity_manager = &mut *engine.getEntityManager();
        let mut camera_entity = Entity::default();
        entity_manager.create(1, &mut camera_entity as *mut _);

        let camera = &mut *engine.createCamera(camera_entity);
        let aspect = 800.0 / 600.0;
        let zoom = 1.0;

        camera.setProjection(
            Camera_Projection::ORTHO,
            -aspect * zoom,
            aspect * zoom,
            -zoom,
            zoom,
            0.0,
            10.0,
        );

        let viewport = Viewport {
            _base: backend::Viewport {
                left: 0,
                bottom: 0,
                width: 800,
                height: 600,
            },
        };

        view.setPostProcessingEnabled(false);
        view.setViewport(&viewport);
        view.setScene(scene);
        view.setCamera(camera);

        let clear_options = Renderer_ClearOptions {
            clearColor: [0.0, 0.0, 1.0, 1.0],
            clear: true,
            discard: true,
        };
        renderer.setClearOptions(&clear_options as *const _);

        let (vertices, indices, texture_data) = triangle_data();

        let mut vertex_buffer_builder = VertexBuffer_Builder::new();
        vertex_buffer_builder.vertexCount(3);
        vertex_buffer_builder.bufferCount(1);
        vertex_buffer_builder.attribute(
            VertexAttribute::POSITION,
            0,
            backend::ElementType::FLOAT2,
            0,
            16,
        );
        vertex_buffer_builder.attribute(
            VertexAttribute::UV0,
            0,
            backend::ElementType::FLOAT2,
            8,
            16,
        );
        let vertex_buffer = &mut *vertex_buffer_builder.build(engine);
        let mut vertices_desc = make_buffer_descriptor(vertices, |_| {});
        vertex_buffer.setBufferAt(engine, 0, &mut vertices_desc, 0);

        let mut index_buffer_builder = IndexBuffer_Builder::new();
        index_buffer_builder.indexCount(3);
        index_buffer_builder.bufferType(IndexBuffer_IndexType::USHORT);
        let index_buffer = &mut *index_buffer_builder.build(engine);
        let mut indices_desc = make_buffer_descriptor(indices, |_| {});
        index_buffer.setBuffer(engine, &mut indices_desc, 0);

        let mut texture_builder = Texture_Builder::new();
        texture_builder.width(256);
        texture_builder.height(256);
        texture_builder.format(backend::TextureFormat::RGB8);
        let texture = &mut *texture_builder.build(engine);
        let mut texture_desc = make_pixel_buffer_descriptor(
            texture_data,
            backend::PixelDataFormat::RGB,
            backend::PixelDataType::UBYTE,
            1,
            0,
            0,
            0,
            |_| {},
        );
        texture.setImage(engine, 0, &mut texture_desc);

        let mut material_builder = Material_Builder::new();
        material_builder.package(
            MATERIAL_BYTES.as_ptr() as *const _,
            MATERIAL_BYTES.len(),
        );
        let material = &mut *material_builder.build(engine);
        let material_instance = &mut *material.getDefaultInstance();
        let name = CString::new("texture").unwrap();
        material_instance.setParameter(
            name.as_ptr(),
            texture,
            &TextureSampler {
                mSamplerParams: SamplerParams::default(),
            },
        );

        let mut renderable = Entity::default();
        entity_manager.create(1, &mut renderable);

        let mut renderable_manager_builder = RenderableManager_Builder::new(1);
        renderable_manager_builder.culling(false);
        renderable_manager_builder.castShadows(false);
        renderable_manager_builder.receiveShadows(false);
        renderable_manager_builder.material(0, material_instance);
        renderable_manager_builder.geometry2(
            0,
            backend::PrimitiveType::TRIANGLES,
            vertex_buffer,
            index_buffer,
        );
        renderable_manager_builder.build(engine, renderable);

        scene.addEntity(renderable);

        renderer.beginFrame(swap_chain, 0);

        println!("start rendering");

        renderer.render(view);
        const BYTE_COUNT: usize = 800 * 600 * 4;
        let buffer = vec![0u8; BYTE_COUNT];
        let mut pixel = make_pixel_buffer_descriptor(
            buffer,
            backend::PixelDataFormat::RGBA,
            backend::PixelDataType::UBYTE,
            1,
            0,
            0,
            0,
            pixelbuffer_read_callback,
        );
        renderer.readPixels(0, 0, 800, 600, &mut pixel as *mut _);

        renderer.endFrame();

        engine.flushAndWait();

        Engine::destroy1(engine);
    }
}

fn pixelbuffer_read_callback(buffer: &mut Vec<u8>) {
    unsafe {
        convert_rgba_to_rgb(buffer.as_mut_ptr(), 800, 600);
    }

    let slice_u8 = &buffer[..800 * 600 * 3];

    let mut file = File::create("triangle.ppm").unwrap();
    println!("The rendering result is written to triangle.ppm");
    file.write(format!("P6 {} {} {}\n", 800, 600, 255).as_bytes())
        .unwrap();
    file.write_all(slice_u8).unwrap();
    file.flush().unwrap();
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
    callback: impl FnOnce(&mut Vec<u8>),
) -> BufferDescriptor {
    let callback_box: Box<Box<dyn FnOnce(&mut Vec<u8>)>> = Box::new(Box::new(callback));
    let user = Box::into_raw(callback_box);
    let desc = BufferDescriptor {
        buffer: data.as_mut_ptr() as *mut _,
        size: (data.len() * std::mem::size_of::<T>()).try_into().unwrap(),
        mCallback: Some(buffer_descriptor_callback),
        mUser: user as *mut _,
        mHandler: ptr::null_mut(),
    };
    std::mem::forget(data);
    desc
}

fn make_pixel_buffer_descriptor<T: Sized>(
    data: Vec<T>,
    format: backend::PixelDataFormat,
    datatype: backend::PixelDataType,
    alignment: u8,
    left: u32,
    top: u32,
    stride: u32,
    callback: impl FnOnce(&mut Vec<u8>),
) -> backend::PixelBufferDescriptor {
    backend::PixelBufferDescriptor {
        _base: make_buffer_descriptor(data, callback),
        left: left,
        top: top,
        __bindgen_anon_1: backend::PixelBufferDescriptor__bindgen_ty_1 {
            __bindgen_anon_1: backend::PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1 {
                stride: stride,
                format: format.into(),
            },
        },
        _bitfield_1: backend::PixelBufferDescriptor::new_bitfield_1(datatype.into(), alignment),
        ..Default::default()
    }
}

pub unsafe extern "C" fn buffer_descriptor_callback(
    ptr: *mut std::ffi::c_void,
    size: usize,
    user: *mut std::ffi::c_void,
) {
    let mut buffer: Vec<u8> = Vec::from_raw_parts(ptr as *mut _, size as usize, size as usize);

    if !user.is_null() {
        let user_fn: Box<Box<dyn FnOnce(&mut Vec<u8>)>> = Box::from_raw(user as *mut _);
        user_fn(&mut buffer);
    }

    std::mem::drop(buffer);
}
