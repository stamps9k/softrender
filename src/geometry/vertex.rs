use crate::math::Vec3;

/// A vertex in model space, forming the basic building block of geometry.
///
/// Fields beyond `position` (texture coordinates, normals, vertex colours)
/// will be added here in later milestones without requiring changes to
/// [`Mesh`] or any triangle representation.
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
  /// Position of the vertex in model space.
  pub position: Vec3,
}

impl Vertex {
  /// Creates a new [`Vertex`] at the given position.
  ///
  /// # Arguments
  ///
  /// * `position` - The vertex position in model space.
  #[must_use]
  pub fn new(position: Vec3) -> Self {
    Self { position }
  }
}
