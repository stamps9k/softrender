use crate::geometry::Vertex;
use crate::math::{Mat4, Vec3};

/// A collection of triangles in model space, with an associated model-to-world
/// transform.
///
/// Each triangle is represented as three [`Vertex`] values in counter-clockwise
/// winding order when viewed from outside the surface.
pub struct Mesh {
  /// The triangles that make up this mesh.
  pub triangles: Vec<[Vertex; 3]>,
  /// The model-to-world transform applied to this mesh during rendering.
  pub transform: Mat4,
}

impl Mesh {
  /// Creates a new [`Mesh`] from a list of triangles, with an identity
  /// transform.
  ///
  /// # Arguments
  ///
  /// * `triangles` - Triangles in counter-clockwise winding order.
  #[must_use]
  pub fn new(triangles: Vec<[Vertex; 3]>) -> Self {
    Self {
      triangles,
      transform: Mat4::identity(),
    }
  }

  /// Creates a unit cube centred at the origin.
  ///
  /// The cube spans from -0.5 to 0.5 on all axes. All triangles use
  /// counter-clockwise winding order when viewed from outside the surface.
  #[must_use]
  pub fn cube() -> Self {
    let v = |x, y, z| Vertex::new(Vec3::new(x, y, z));

    // Eight corners
    let lbf = v(-0.5, -0.5, 0.5); // left  bottom front
    let rbf = v(0.5, -0.5, 0.5); // right bottom front
    let rtf = v(0.5, 0.5, 0.5); // right top    front
    let ltf = v(-0.5, 0.5, 0.5); // left  top    front
    let lbb = v(-0.5, -0.5, -0.5); // left  bottom back
    let rbb = v(0.5, -0.5, -0.5); // right bottom back
    let rtb = v(0.5, 0.5, -0.5); // right top    back
    let ltb = v(-0.5, 0.5, -0.5); // left  top    back

    Self::new(vec![
      // Front face  (+z)
      [lbf, rbf, rtf],
      [lbf, rtf, ltf],
      // Back face   (-z)
      [rbb, lbb, ltb],
      [rbb, ltb, rtb],
      // Left face   (-x)
      [lbb, lbf, ltf],
      [lbb, ltf, ltb],
      // Right face  (+x)
      [rbf, rbb, rtb],
      [rbf, rtb, rtf],
      // Top face    (+y)
      [ltf, rtf, rtb],
      [ltf, rtb, ltb],
      // Bottom face (-y)
      [lbb, rbb, rbf],
      [lbb, rbf, lbf],
    ])
  }
}
