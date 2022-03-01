use filament_bindings::{
    backend,
    filament::{
        Bounds, Engine, IndexBufferBuilder, IndexType, MaterialBuilder, RenderableBuilder, RgbType,
        VertexAttribute, VertexBufferBuilder, LightBuilder, LightType,
    },
    filamesh::MeshReader,
    math::Float3,
};
use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use std::ptr;

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
const RESOURCES_AIDEFAULTMAT_DATA: &'static [u8] = include_bytes!("aiDefaultMat_ogl.filament");
const MONKEY_DATA: &'static [u8] = include_bytes!("monkey.filamesh");

fn main() {
    let (event_loop, window, surface) = init_window();

    unsafe {
        let engine = Engine::create(backend::Backend::OPENGL).unwrap();
        let scene = engine.create_scene().unwrap();
        let entity_manager = engine.get_entity_manager().unwrap();

        let mut triangle = entity_manager.create();
        {
            scene.add_entity(&triangle);
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
                let mut vertex_buffer_builder = VertexBufferBuilder::new();
                vertex_buffer_builder.vertex_count(3);
                vertex_buffer_builder.buffer_count(2);
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
                    backend::ElementType::UBYTE4,
                    0,
                    4,
                );
                vertex_buffer_builder.normalized(VertexAttribute::COLOR, true);
                vertex_buffer_builder.build(&mut engine).unwrap()
            };
            vertex_buffer.set_buffer_at(
                &mut engine,
                0,
                backend::BufferDescriptor::new(triangle_position),
                0,
            );
            vertex_buffer.set_buffer_at(
                &mut engine,
                1,
                backend::BufferDescriptor::new(triangle_color),
                0,
            );

            let index_buffer = {
                let mut index_buffer_builder = IndexBufferBuilder::new();
                index_buffer_builder.index_count(3);
                index_buffer_builder.buffer_type(IndexType::USHORT);
                index_buffer_builder.build(&mut engine).unwrap()
            };
            index_buffer.set_buffer(
                &mut engine,
                backend::BufferDescriptor::new(vec![0u16, 1u16, 2u16]),
                0,
            );

            let mut material_builder = MaterialBuilder::new();
            material_builder.package(MATERIAL_BYTES);
            let material = material_builder.build(&mut engine).unwrap();
            let material_instance = material.get_default_instance().unwrap();

            let mut renderable_manager_builder = RenderableBuilder::new(1);
            renderable_manager_builder.bounding_box(&mut Bounds {
                center: Float3::new(-1.0, -1.0, -1.0),
                half_extent: Float3::new(1.0, 1.0, 1.0),
            });
            renderable_manager_builder.material(0, &mut material_instance);
            renderable_manager_builder.geometry(
                0,
                backend::PrimitiveType::TRIANGLES,
                &mut vertex_buffer,
                &mut index_buffer,
            );
            renderable_manager_builder.build(&mut engine, &triangle);
        }

        // monkey
        {
            let mut material_builder = MaterialBuilder::new();
            material_builder.package(RESOURCES_AIDEFAULTMAT_DATA);
            let material = material_builder.build(&mut engine).unwrap();
            let material_instance = material.create_instance().unwrap();
            material_instance.set_rgb_parameter(
                "baseColor",
                RgbType::LINEAR,
                Float3::new(0.8, 0.8, 0.8),
            );
            material_instance.set_float_parameter("metallic", &1.0);
            material_instance.set_float_parameter("roughness", &0.4);
            material_instance.set_float_parameter("reflectance", &0.5);

            let mesh = MeshReader::load_mesh_from_buffer_default_material(
                &mut engine,
                Vec::from(MONKEY_DATA),
                &mut material_instance,
            ).unwrap();
            scene.add_entity(&mesh.renderable);

            let mut light = entity_manager.create();
            let mut light_builder = LightBuilder::new(LightType::SUN);
            light_builder.color(&helper_color_toLinear_fast_sRGB(&[0.98, 0.92, 0.89]));
            light_builder.intensity(110000.0);
            light_builder.direction(&[0.7, -1.0, -0.8]);
            light_builder.sunAngularRadius(1.9);
            light_builder.castShadows(false);
            light_builder.build(engine, light);
            scene.addEntity(light);
        }

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
                width: window.inner_size().width,
                height: window.inner_size().height,
            },
        };

        let aspect = window.inner_size().width as f64 / window.inner_size().height as f64;

        view.setViewport(&viewport);
        camera.setLensProjection(28.0, aspect, 0.1, 100.0);
        camera.lookAt(&[4.0, 0.0, 4.0], &[0.0, 0.0, 0.0], &[0.0, 1.0, 0.0]);

        let tcm = &mut *engine.getTransformManager();
        let inst = tcm.getInstance(triangle);
        tcm.setTransform(
            inst,
            &[
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );

        // let mut b = view.getBloomOptions();
        // b.enabled = true;
        // view.setBloomOptions(b);

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
                    let viewport = Viewport {
                        _base: backend::Viewport {
                            left: 0,
                            bottom: 0,
                            width: window.inner_size().width,
                            height: window.inner_size().height,
                        },
                    };

                    let aspect =
                        window.inner_size().width as f64 / window.inner_size().height as f64;

                    view.setViewport(&viewport);
                    camera.setLensProjection(28.0, aspect, 0.1, 100.0);

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
