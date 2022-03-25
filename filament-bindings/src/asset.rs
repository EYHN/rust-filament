use crate::{filament, utils, math::Float3};

pub trait Asset {
    fn get_renderables(&self) -> &[utils::Entity];
    fn get_root_entity(&self) -> &utils::Entity;
    fn get_aabb(&self) -> &filament::Aabb;
    fn get_main_camera(&self) -> Option<&CameraInfo>;
    fn destory(&mut self, engine: &mut filament::Engine);
}

pub struct CameraInfo {
    pub position: Float3,
    pub up: Float3,
    pub look_at: Float3,
    pub horizontal_fov: f64,
    pub aspect: f64,
    pub orthographic_width: f64,
}
