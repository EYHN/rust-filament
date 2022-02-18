use filament_safe::{
    backend::Backend,
    filament::{Engine, Renderer, SwapChain, SwapChainConfig, Camera, Projection, View},
};

fn main() {
    let mut engine = Engine::create(Backend::OPENGL).unwrap();
    let swap_chain =
        SwapChain::create_headless_swap_chain(&mut engine, 800, 600, Default::default()).unwrap();
    let renderer = Renderer::create(&mut engine).unwrap();
    let view = View::create(&mut engine, &swap_chain).unwrap();
    // let scene = engine.create_scene().unwrap();

    let mut entity_manager = engine.get_entity_manager();
    let camera_entity = entity_manager.create();

    let mut camera = Camera::create(&mut engine, &camera_entity).unwrap();

    camera.set_projection(
        Projection::ORTHO,
        0.1,
        100.0,
        0.1,
        100.0,
        0.1,
        100.0,
    );

    // let camera = engine.create_camera().unwrap();
}
