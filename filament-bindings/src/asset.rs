use crate::{filament, utils};

pub trait Asset {
    fn get_renderables(&self) -> &[utils::Entity];
    fn get_root_entity(&self) -> &utils::Entity;
    fn get_aabb(&self) -> &filament::Aabb;
    fn destory(self, engine: &mut filament::Engine);
}
