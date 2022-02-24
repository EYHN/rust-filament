use filament_bindings::filament_math_float4;
use filament_safe::{
    backend::{
        Backend, BufferDescriptor, ElementType, PixelBufferDescriptor, PixelDataFormat,
        PixelDataType, PrimitiveType, TextureFormat,
    },
    filament::{
        ClearOptions, Engine, IndexBufferBuilder, IndexType, MaterialBuilder, Projection,
        RenderableOptions, Renderer, Scene, SwapChain, TextureBuilder, TextureSampler,
        VertexAttribute, VertexBufferBuilder, View, Viewport,
    },
};
use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

const MATERIAL_BYTES: &'static [u8] = include_bytes!("texture_unlit_ogl.filamat");

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

// #[cfg(target_os = "linux")]
// fn get_active_surface(window: &winit::window::Window) -> *mut std::ffi::c_void {
//     use winit::platform::unix::WindowExtUnix;
//     window.xlib_window().unwrap() as *mut std::ffi::c_void
// }

fn init_window() -> (EventLoop<()>, Window, *mut std::ffi::c_void) {
    let event_loop = EventLoop::<()>::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Basic example window");

    let surface = get_active_surface(&window);

    (event_loop, window, surface)
}

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
    let (event_loop, window, surface) = init_window();

    let mut engine = Engine::create(Backend::OPENGL).unwrap();
    let mut swapchain =
        SwapChain::create_swap_chain(&mut engine, surface, Default::default()).unwrap();
    let mut renderer = Renderer::create(&mut engine).unwrap();
    let mut view = View::create(&mut engine).unwrap();
    view.set_post_processing_enabled(false);
    let mut scene = Scene::create(&mut engine).unwrap();

    {
        renderer.set_clear_options(&ClearOptions {
            clear_color: filament_math_float4 {
                inner: [0.0, 0.0, 1.0, 1.0],
            },
            clear: true,
            discard: true,
        });

        let (vertices, indices, texture_data) = triangle_data();
        let mut vertex_buffer = VertexBufferBuilder::new()
            .vertex_count(3)
            .buffer_count(1)
            .attribute(VertexAttribute::POSITION, 0, ElementType::FLOAT2, 0, 16)
            .attribute(VertexAttribute::UV0, 0, ElementType::FLOAT2, 8, 16)
            .build(&mut engine)
            .unwrap();
        vertex_buffer
            .set_buffer_at(0, BufferDescriptor::new(vertices), 0)
            .unwrap();

        let mut index_buffer = IndexBufferBuilder::new()
            .index_count(3)
            .buffer_type(IndexType::USHORT)
            .build(&mut engine)
            .unwrap();
        index_buffer
            .set_buffer(BufferDescriptor::new(indices), 0)
            .unwrap();

        let mut texture = TextureBuilder::new()
            .width(256)
            .height(256)
            .format(TextureFormat::RGB8)
            .build(&mut engine)
            .unwrap();
        texture
            .set_image(
                0,
                PixelBufferDescriptor::new(
                    texture_data,
                    PixelDataFormat::RGB,
                    PixelDataType::UBYTE,
                ),
            )
            .unwrap();

        let mut material = MaterialBuilder::new()
            .package(MATERIAL_BYTES)
            .build(&mut engine)
            .unwrap()
            .create_instance()
            .unwrap();
        material
            .set_texture_parameter("texture", &texture, &TextureSampler::default())
            .unwrap();

        let entity_manager = engine.get_entity_manager();
        let camera_entity = entity_manager.create();

        let camera = camera_entity.create_camera_component().unwrap();
        let viewport = Viewport {
            left: 0,
            bottom: 0,
            width: 800,
            height: 600,
        };
        let aspect = viewport.width as f64 / viewport.height as f64;
        let zoom = 1.0;

        camera.set_projection(
            Projection::ORTHO,
            -aspect * zoom,
            aspect * zoom,
            -zoom,
            zoom,
            0.0,
            10.0,
        );

        view.set_viewport(&viewport);
        view.set_scene(&mut scene);
        view.set_camera(camera);

        let renderable = entity_manager.create();
        renderable
            .create_renderable_component(
                RenderableOptions::new(1)
                    .culling(false)
                    .cast_shadows(false)
                    .receive_shadows(false)
                    .material(0, &mut material)
                    .geometry(
                        0,
                        PrimitiveType::TRIANGLES,
                        &mut vertex_buffer,
                        &mut index_buffer,
                    ),
            )
            .unwrap();

        scene.add_entity(&renderable);
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
                if renderer.begin_frame(&mut swapchain) {
                    renderer.render(&view);
                    renderer.end_frame();
                }
            }
            _ => {}
        }
    })
}
