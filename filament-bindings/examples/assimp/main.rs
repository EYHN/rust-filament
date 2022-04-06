use core::time;
use std::thread;

use filament_bindings::{
    assimp::AssimpAsset,
    backend,
    filament::{Engine, Fov, IndirectLightBuilder, Projection, Viewport},
    image::{ktx, KtxBundle},
    math::{Float3, Mat3f},
};

use winit::{
    dpi::LogicalSize,
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
    window.set_inner_size(LogicalSize::new(400.0, 300.0));

    let surface = get_active_surface(&window);

    (event_loop, window, surface)
}

const IDL_TEXTURE_DATA: &'static [u8] = include_bytes!("lightroom_14b_ibl.ktx");
const MODEL_NAME: &'static str = "test_model.fbx";
const MODEL_DATA: &'static [u8] = include_bytes!("test_model.fbx");

fn main() {
    let (event_loop, window, surface) = init_window();

    unsafe {
        let mut engine = Engine::create(backend::Backend::OPENGL).unwrap();
        let mut scene = engine.create_scene().unwrap();
        let mut entity_manager = engine.get_entity_manager().unwrap();

        let asset = AssimpAsset::from_memory(&mut engine, MODEL_DATA, MODEL_NAME).unwrap();

        for entity in asset.get_renderables() {
            scene.add_entity(entity);
        }

        scene.add_entity(asset.get_root_entity());

        let ibl_texture = ktx::create_texture(
            &mut engine,
            KtxBundle::from(IDL_TEXTURE_DATA).unwrap(),
            false,
        )
        .unwrap();

        let mut ibl = IndirectLightBuilder::new()
            .unwrap()
            .reflections(&ibl_texture)
            .intensity(50000.0)
            .rotation(&Mat3f::rotation(-90.0, Float3::new(0.0, 1.0, 0.0)))
            .build(&mut engine)
            .unwrap();
        scene.set_indirect_light(&mut ibl);

        let mut swap_chain = engine
            .create_swap_chain(surface, Default::default())
            .unwrap();
        let mut renderer = engine.create_renderer().unwrap();

        let camera_entity = entity_manager.create();
        let mut camera = engine.create_camera(&camera_entity).unwrap();

        let aspect = window.inner_size().width as f64 / window.inner_size().height as f64;

        let mut view = engine.create_view().unwrap();
        view.set_camera(&mut camera);
        view.set_scene(&mut scene);

        let viewport = Viewport {
            left: 0,
            bottom: 0,
            width: window.inner_size().width,
            height: window.inner_size().height,
        };

        view.set_viewport(&viewport);

        camera.set_exposure_physical(16.0, 1.0 / 125.0, 100.0);

        if let Some(camera_info) = asset.get_main_camera() {
            if camera_info.horizontal_fov != 0.0 {
                camera.set_projection_fov_direction(
                    camera_info.horizontal_fov,
                    aspect,
                    0.1,
                    f64::INFINITY,
                    Fov::HORIZONTAL,
                );
            } else {
                camera.set_projection(
                    Projection::ORTHO,
                    -camera_info.orthographic_width,
                    camera_info.orthographic_width,
                    -camera_info.orthographic_width / aspect,
                    camera_info.orthographic_width / aspect,
                    0.1,
                    100000.0,
                );
            }
            camera.look_at_up(&camera_info.position, &camera_info.look_at, &camera_info.up);
        } else {
            let half_extent = asset.get_aabb().extent();
            camera.set_lens_projection(28.0, aspect, 0.1, f64::INFINITY);
            camera.look_at_up(
                &(asset.get_aabb().center()
                    + Float3::from(((half_extent[0] + half_extent[2]) / 2.0).max(half_extent[1]))
                        * Float3::from([2.5, 1.7, 2.5])),
                &asset.get_aabb().center(),
                &[0.0, 1.0, 0.0].into(),
            );
        }

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
                    if renderer.begin_frame(&mut swap_chain) {
                        renderer.render(&view);
                        renderer.end_frame();
                    }

                    thread::sleep(time::Duration::from_millis(16))
                }
                _ => {}
            }
        });
    }
}
