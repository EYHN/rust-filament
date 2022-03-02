use filament_bindings::{
    backend,
    filament::{Engine, LightBuilder, LightType, LinearColor, MaterialBuilder, RgbType, Viewport},
    filameshio::MeshReader,
};

use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
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

#[cfg(target_os = "linux")]
fn get_active_surface(window: &winit::window::Window) -> *mut std::ffi::c_void {
    use winit::platform::windows::WindowExtUnix;
    window.xlib_window().unwrap() as *mut std::ffi::c_void
}

fn init_window() -> (EventLoop<()>, Window, *mut std::ffi::c_void) {
    let event_loop = EventLoop::<()>::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Basic example window");

    let surface = get_active_surface(&window);

    (event_loop, window, surface)
}

const RESOURCES_AIDEFAULTMAT_DATA: &'static [u8] = include_bytes!("aiDefaultMat_ogl.filament");
const MONKEY_DATA: &'static [u8] = include_bytes!("monkey.filamesh");

fn main() {
    let (event_loop, window, surface) = init_window();

    unsafe {
        let mut engine = Engine::create(backend::Backend::OPENGL).unwrap();
        let mut scene = engine.create_scene().unwrap();
        let mut entity_manager = engine.get_entity_manager().unwrap();

        {
            let mut material_builder = MaterialBuilder::new();
            material_builder.package(RESOURCES_AIDEFAULTMAT_DATA);
            let material = material_builder.build(&mut engine).unwrap();
            let mut material_instance = material.create_instance().unwrap();
            material_instance
                .set_rgb_parameter("baseColor", RgbType::LINEAR, [0.8, 0.8, 0.8].into())
                .unwrap()
                .set_float_parameter("metallic", &1.0)
                .unwrap()
                .set_float_parameter("roughness", &0.4)
                .unwrap()
                .set_float_parameter("reflectance", &0.5)
                .unwrap();

            let mesh = MeshReader::load_mesh_from_buffer_default_material(
                &mut engine,
                MONKEY_DATA,
                &mut material_instance,
            )
            .unwrap();
            scene.add_entity(&mesh.renderable);

            let light = entity_manager.create();
            let mut light_builder = LightBuilder::new(LightType::SUN);
            light_builder.color(&LinearColor([0.98, 0.92, 0.89].into()));
            light_builder.intensity(110000.0);
            light_builder.direction(&[0.7, -1.0, -0.8].into());
            light_builder.sun_angular_radius(1.9);
            light_builder.cast_shadows(false);
            light_builder.build(&mut engine, &light);
            scene.add_entity(&light);
        }

        let mut swap_chain = engine
            .create_swap_chain(surface, Default::default())
            .unwrap();
        let mut renderer = engine.create_renderer().unwrap();

        let camera_entity = entity_manager.create();
        let mut camera = engine.create_camera(&camera_entity).unwrap();

        let mut view = engine.create_view().unwrap();
        view.set_camera(&mut camera);
        view.set_scene(&mut scene);

        let viewport = Viewport {
            left: 0,
            bottom: 0,
            width: window.inner_size().width,
            height: window.inner_size().height,
        };

        let aspect = window.inner_size().width as f64 / window.inner_size().height as f64;

        view.set_viewport(&viewport);
        camera.set_lens_projection(28.0, aspect, 0.1, 100.0);
        camera.look_at_up(
            &[4.0, 0.0, 4.0].into(),
            &[0.0, 0.0, 0.0].into(),
            &[0.0, 1.0, 0.0].into(),
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
                    let viewport = Viewport {
                        left: 0,
                        bottom: 0,
                        width: window.inner_size().width,
                        height: window.inner_size().height,
                    };

                    let aspect =
                        window.inner_size().width as f64 / window.inner_size().height as f64;

                    view.set_viewport(&viewport);
                    camera.set_lens_projection(28.0, aspect, 0.1, 100.0);

                    if renderer.begin_frame(&mut swap_chain) {
                        renderer.render(&view);
                        renderer.end_frame();
                    }
                }
                _ => {}
            }
        })
    }
}
