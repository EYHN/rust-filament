use std::{ptr::NonNull, rc::Rc};

use filament_bindings::filament_Scene;

use super::Engine;

pub struct Scene(Rc<NonNull<filament_Scene>>, Engine);