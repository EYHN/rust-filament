use std::mem;

use filament_bindings::filament_math_float4;
use filament_safe::{
    backend::Backend,
    filament::{ClearOptions, Engine, Projection, Renderer, Scene, SwapChain, View, Viewport},
};

fn main() {
    let mut engine = Engine::create(Backend::OPENGL).unwrap();
    let mut swap_chain =
        SwapChain::create_headless_swap_chain(&mut engine, 800, 600, Default::default()).unwrap();
    let mut renderer = Renderer::create(&mut engine).unwrap();
    let mut view = View::create(&mut engine).unwrap();
    let mut scene = Scene::create(&mut engine).unwrap();

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

    renderer.set_clear_options(&ClearOptions {
        clear_color: filament_math_float4 {
            inner: [0.0, 0.0, 1.0, 1.0],
        },
        clear: true,
        discard: true,
    });

    renderer.begin_frame(&mut swap_chain);

    renderer.render(&view);

    renderer.end_frame();

    engine.flush_and_wait();

    mem::drop(engine);

    println!("end")

    // let camera = engine.create_camera().unwrap();
}
