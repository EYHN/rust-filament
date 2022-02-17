use filament_bindings::{filament_Engine_destroy12, filament_Scene};

use super::{EngineDestroy, EngineManaged};

pub type Scene<'a> = EngineManaged<'a, filament_Scene>;

impl EngineDestroy for filament_Scene {
    fn destory(p: *const Self, engine: &mut super::Engine) -> bool {
        unsafe { filament_Engine_destroy12(engine.0, p) }
    }
}
