use filament_bindings::root::{
    filament::{
        self,
        backend::{self, Backend, BufferDescriptor},
        Engine, IndexBuffer_Builder, IndexBuffer_IndexType, LightManager_Builder,
        LightManager_Type, Material_Builder, RenderableManager_Builder, Renderer_ClearOptions,
        RgbType, VertexAttribute, VertexBuffer_Builder, Viewport,
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
const RESOURCES_AIDEFAULTMAT_DATA: &'static [u8] = include_bytes!("aiDefaultMat_ogl.filament");
const MONKEY_DATA: &'static [u8] = include_bytes!("monkey.filamesh");

fn main() {
    let (event_loop, window, surface) = init_window();

    unsafe {
        let engine = &mut *Engine::create(Backend::OPENGL, null_mut(), null_mut());
        let scene = &mut *engine.createScene();
        let entity_manager = &mut *engine.getEntityManager();

        let mut triangle = Entity::default();
        entity_manager.create(1, &mut triangle as *mut _);
        {
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
                    backend::ElementType::UBYTE4,
                    0,
                    4,
                );
                vertex_buffer_builder.normalized(VertexAttribute::COLOR, true);
                &mut *vertex_buffer_builder.build(engine)
            };
            let mut bd = make_buffer_descriptor(triangle_position, |_| {});
            vertex_buffer.setBufferAt(engine, 0, &mut bd, 0);
            let mut bd = make_buffer_descriptor(triangle_color, |_| {});
            vertex_buffer.setBufferAt(engine, 1, &mut bd, 0);

            let index_buffer = {
                let mut index_buffer_builder = IndexBuffer_Builder::new();
                index_buffer_builder.indexCount(3);
                index_buffer_builder.bufferType(IndexBuffer_IndexType::USHORT);
                &mut *index_buffer_builder.build(engine)
            };
            let mut bd = make_buffer_descriptor(vec![0u16, 1u16, 2u16], |_| {});
            index_buffer.setBuffer(engine, &mut bd, 0);

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
        }

        // monkey
        {
            let mut material_builder = Material_Builder::new();
            material_builder.package(
                RESOURCES_AIDEFAULTMAT_DATA.as_ptr() as *const _,
                RESOURCES_AIDEFAULTMAT_DATA.len(),
            );
            let material = &*material_builder.build(engine);
            let material_instance = &mut *material.createInstance(ptr::null());
            let parameter_name = CString::new("baseColor").unwrap();
            material_instance.setParameter1(
                parameter_name.as_ptr(),
                RgbType::LINEAR,
                [0.8, 0.8, 0.8],
            );
            let parameter_name = CString::new("metallic").unwrap();
            helper_material_instance_setParameter_float(
                material_instance,
                parameter_name.as_ptr(),
                &1.0,
            );
            let parameter_name = CString::new("roughness").unwrap();
            helper_material_instance_setParameter_float(
                material_instance,
                parameter_name.as_ptr(),
                &0.4,
            );
            let parameter_name = CString::new("reflectance").unwrap();
            helper_material_instance_setParameter_float(
                material_instance,
                parameter_name.as_ptr(),
                &0.5,
            );

            let mesh = MeshReader::loadMeshFromBuffer1(
                engine,
                MONKEY_DATA.as_ptr() as *const _,
                None,
                ptr::null_mut(),
                material_instance,
            );
            scene.addEntity(mesh.renderable);

            let mut light = Entity::default();
            entity_manager.create(1, &mut light as *mut _);
            let mut light_builder = LightManager_Builder::new(LightManager_Type::SUN);
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
