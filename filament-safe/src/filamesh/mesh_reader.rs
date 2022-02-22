use filament_bindings::filamesh_MeshReader;

use crate::filament::{MaterialInstance, Engine};

pub struct MeshReader {
  native: filamesh_MeshReader
}

impl MeshReader {
  pub fn loadMeshFromBuffer(engine: &mut Engine, buffer: Vec<u8>, material: &MaterialInstance) -> Result<MeshReader, String> {
    todo!()
  }
}
