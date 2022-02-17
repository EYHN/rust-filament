use filament_bindings::filament_View;

use super::EngineManaged;

pub type View<'a> = EngineManaged<'a, filament_View>;
