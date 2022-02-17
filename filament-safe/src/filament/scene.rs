use filament_bindings::filament_Scene;

use super::EngineManaged;

pub type Scene<'a> = EngineManaged<'a, filament_Scene>;
