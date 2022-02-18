use filament_bindings::utils_Entity;

use super::EntityManager;

pub struct Entity<'a>(pub &'a EntityManager, pub(crate) utils_Entity);
