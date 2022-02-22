use filament_bindings::root::{
    filament::{
        backend::{self, Backend, BufferDescriptor, SamplerParams},
        Camera_Projection, Engine, IndexBuffer_Builder, IndexBuffer_IndexType, Material_Builder,
        RenderableManager_Builder, Renderer_ClearOptions, RgbType, TextureSampler, Texture_Builder,
        VertexAttribute, VertexBuffer_Builder, Viewport,
    },
    utils::Entity,
};
use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use std::{
    ffi::CString,
    fs::File,
    io::Write,
    ptr::{self, null_mut},
};

#[cfg(target_os = "macos")]
fn get_active_surface(window: &winit::window::Window) -> *mut std::ffi::c_void {
    use winit::platform::macos::WindowExtMacOS;
    window.ns_view()
}

#[cfg(target_os = "windows")]
fn get_active_surface(window: &winit::window::Window) -> *mut std::ffi::c_void {
    use winit::platform::windows::WindowExtWindows;
    window.hwnd()
}

fn init_window() -> (EventLoop<()>, Window, *mut std::ffi::c_void) {
    let event_loop = EventLoop::<()>::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Basic example window");

    let surface = get_active_surface(&window);

    (event_loop, window, surface)
}

const RESOURCES_AIDEFAULTMAT_DATA: &'static [u8] = include_bytes!("aiDefaultMat_ogl.filament");

fn main() {
    let (event_loop, window, surface) = init_window();

    unsafe {
        let engine = &mut *Engine::create(Backend::OPENGL, null_mut(), null_mut());
        let swap_chain = engine.createSwapChain(surface, 0);
        let renderer = &mut *engine.createRenderer();
        let view = &mut *engine.createView();
        let scene = &mut *engine.createScene();
        let transform_manager = &mut *engine.getTransformManager();
        let renderable_manager = &mut *engine.getRenderableManager();

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

        view.setViewport(&viewport);
        view.setScene(scene);
        view.setCamera(camera);

        let mut material_builder = Material_Builder::new();
        material_builder.package(
            RESOURCES_AIDEFAULTMAT_DATA.as_ptr() as *const _,
            RESOURCES_AIDEFAULTMAT_DATA.len() as u64,
        );
        let material = &mut *material_builder.build(engine);
        let material_instance = &mut *material.createInstance(ptr::null());
        let parameter_name = CString::new("baseColor").unwrap();
        material_instance.setParameter1(parameter_name.as_ptr(), RgbType::LINEAR, [0.8, 0.8, 0.8]);
        let parameter_name = CString::new("metallic").unwrap();
        material_instance.setParameter(parameter_name.as_ptr(), 1.0);

        ///////////////////////////////////////// todo here

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

        let mut close_requested = false;

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
            use winit::event::{ElementState, VirtualKeyCode};

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        close_requested = true;
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(virtual_code),
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => match virtual_code {
                        VirtualKeyCode::Escape => close_requested = true,
                        _ => (),
                    },
                    _ => (),
                },
                Event::MainEventsCleared => {
                    window.request_redraw();

                    if close_requested {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                Event::RedrawRequested(_window_id) => {
                    if renderer.beginFrame(swap_chain, 0) {
                        renderer.render(view);
                        renderer.endFrame();
                    }
                }
                _ => {}
            }
        })
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
    size: u64,
    user: *mut std::ffi::c_void,
) {
    let mut buffer: Vec<u8> = Vec::from_raw_parts(ptr as *mut _, size as usize, size as usize);

    if !user.is_null() {
        let user_fn: Box<Box<dyn FnOnce(&mut Vec<u8>)>> = Box::from_raw(user as *mut _);
        user_fn(&mut buffer);
    }

    std::mem::drop(buffer);
}
