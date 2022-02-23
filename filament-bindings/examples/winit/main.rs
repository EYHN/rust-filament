use filament_bindings::root::{
    filament::{
        self,
        backend::{self, Backend, BufferDescriptor},
        Camera_Projection, Color, Engine, IndexBuffer_Builder, IndexBuffer_IndexType,
        LightManager_Builder, LightManager_Type, Material_Builder, RenderableManager_Builder,
        Renderer_ClearOptions, RgbType, VertexAttribute, VertexBuffer_Builder, Viewport,
    },
    filamesh::MeshReader,
    helper_color_toLinear_fast_sRGB, helper_material_instance_setParameter_float,
    utils::Entity,
};
use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use std::{
    ffi::CString,
    ptr::{self, null_mut},
    thread::{self, Thread},
    time::Duration,
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

const MATERIAL_BYTES: &'static [u8] = include_bytes!("bakedColor.filamat");

fn main() {
    let (event_loop, window, surface) = init_window();

    unsafe {
        let engine = &mut *Engine::create(Backend::OPENGL, null_mut(), null_mut());
        let scene = &mut *engine.createScene();
        let entity_manager = &mut *engine.getEntityManager();

        let mut triangle = Entity::default();
        entity_manager.create(1, &mut triangle as *mut _);

        scene.addEntity(triangle);
        let triangle_position: Vec<f32> = vec![
            1.0,
            0.0,
            f32::cos(std::f32::consts::PI * 2.0 / 3.0),
            f32::sin(std::f32::consts::PI * 2.0 / 3.0),
            f32::cos(std::f32::consts::PI * 4.0 / 3.0),
            f32::sin(std::f32::consts::PI * 4.0 / 3.0),
        ];
        let triangle_color: Vec<u32> = vec![0xffff0000, 0xff00ff00, 0xff0000ff];

        let vertex_buffer = {
            let mut vertex_buffer_builder = VertexBuffer_Builder::new();
            vertex_buffer_builder.vertexCount(3);
            vertex_buffer_builder.bufferCount(2);
            vertex_buffer_builder.attribute(
                VertexAttribute::POSITION,
                0,
                backend::ElementType::FLOAT2,
                0,
                8,
            );
            vertex_buffer_builder.attribute(
                VertexAttribute::COLOR,
                1,
                backend::ElementType::FLOAT2,
                0,
                4,
            );
            &mut *vertex_buffer_builder.build(engine)
        };
        vertex_buffer.setBufferAt(
            engine,
            0,
            &mut make_buffer_descriptor(triangle_position, |_| {}),
            0,
        );

        vertex_buffer.setBufferAt(
            engine,
            1,
            &mut make_buffer_descriptor(triangle_color, |_| {}),
            0,
        );

        let index_buffer = {
            let mut index_buffer_builder = IndexBuffer_Builder::new();
            index_buffer_builder.indexCount(3);
            index_buffer_builder.bufferType(IndexBuffer_IndexType::USHORT);
            &mut *index_buffer_builder.build(engine)
        };
        index_buffer.setBuffer(
            engine,
            &mut make_buffer_descriptor(vec![0, 1, 2], |_| {}),
            0,
        );

        let mut material_builder = Material_Builder::new();
        material_builder.package(MATERIAL_BYTES.as_ptr() as *const _, MATERIAL_BYTES.len());
        let material = &mut *material_builder.build(engine);
        let material_instance = &mut *material.getDefaultInstance();

        let mut renderable_manager_builder = RenderableManager_Builder::new(1);
        renderable_manager_builder.boundingBox(&mut filament::Box {
            center: [-1.0, -1.0, -1.0],
            halfExtent: [1.0, 1.0, 1.0],
        });
        renderable_manager_builder.material(0, material_instance);
        renderable_manager_builder.geometry2(
            0,
            backend::PrimitiveType::TRIANGLES,
            vertex_buffer,
            index_buffer,
        );
        renderable_manager_builder.build(engine, triangle);

        let swap_chain = &mut *engine.createSwapChain(surface, 0);
        let renderer = &mut *engine.createRenderer();

        let mut camera_entity = Entity::default();
        entity_manager.create(1, &mut camera_entity as *mut _);
        let camera = &mut *engine.createCamera(camera_entity);

        let view = &mut *engine.createView();
        view.setCamera(camera);
        view.setScene(scene);

        renderer.setClearOptions(&Renderer_ClearOptions {
            clearColor: [0.0, 0.1, 0.2, 1.0],
            clear: true,
            discard: true,
        });

        let viewport = Viewport {
            _base: backend::Viewport {
                left: 0,
                bottom: 0,
                width: 800,
                height: 600,
            },
        };

        let aspect = 800 as f64 / 600 as f64;

        view.setViewport(&viewport);
        camera.setProjection(
            Camera_Projection::ORTHO,
            -aspect,
            aspect,
            -1.0,
            1.0,
            0.0,
            1.0,
        );

        let tcm = &mut *engine.getTransformManager();
        let inst = tcm.getInstance(triangle);
        tcm.setTransform(
            inst,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

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

// fn make_buffer_descriptor<T: Sized>(
//     mut data: Vec<T>,
//     callback: impl FnOnce(&mut Vec<u8>),
// ) -> BufferDescriptor {
//     let callback_box: Box<Box<dyn FnOnce(&mut Vec<u8>)>> = Box::new(Box::new(callback));
//     let user = Box::into_raw(callback_box);
//     let desc = BufferDescriptor {
//         buffer: data.as_mut_ptr() as *mut _,
//         size: (data.len() * std::mem::size_of::<T>()).try_into().unwrap(),
//         mCallback: Some(buffer_descriptor_callback),
//         mUser: user as *mut _,
//         mHandler: ptr::null_mut(),
//     };
//     std::mem::forget(data);
//     desc
// }

// fn make_pixel_buffer_descriptor<T: Sized>(
//     data: Vec<T>,
//     format: backend::PixelDataFormat,
//     datatype: backend::PixelDataType,
//     alignment: u8,
//     left: u32,
//     top: u32,
//     stride: u32,
//     callback: impl FnOnce(&mut Vec<u8>),
// ) -> backend::PixelBufferDescriptor {
//     backend::PixelBufferDescriptor {
//         _base: make_buffer_descriptor(data, callback),
//         left: left,
//         top: top,
//         __bindgen_anon_1: backend::PixelBufferDescriptor__bindgen_ty_1 {
//             __bindgen_anon_1: backend::PixelBufferDescriptor__bindgen_ty_1__bindgen_ty_1 {
//                 stride: stride,
//                 format: format.into(),
//             },
//         },
//         _bitfield_1: backend::PixelBufferDescriptor::new_bitfield_1(datatype.into(), alignment),
//         ..Default::default()
//     }
// }

// pub unsafe extern "C" fn buffer_descriptor_callback(
//     ptr: *mut std::ffi::c_void,
//     size: usize,
//     user: *mut std::ffi::c_void,
// ) {
//     let mut buffer: Vec<u8> = Vec::from_raw_parts(ptr as *mut _, size as usize, size as usize);

//     if !user.is_null() {
//         let user_fn: Box<Box<dyn FnOnce(&mut Vec<u8>)>> = Box::from_raw(user as *mut _);
//         user_fn(&mut buffer);
//     }

//     std::mem::drop(buffer);
// }

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
