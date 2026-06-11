use crate::math::{Vec3, Vec4};

/// A 4x4 matrix of `f32` values, stored in row-major order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
  pub data: [[f32; 4]; 4],
}

impl Mat4 {
  /// Creates a new `Mat4` from a 2D array of `f32` values.
  ///
  /// Values are stored in row-major order, meaning `data[row][column]`.
  ///
  /// # Arguments
  ///
  /// * `data` - A 4x4 array of `f32` values in row-major order
  #[must_use]
  pub fn new(data: [[f32; 4]; 4]) -> Self {
    Self { data }
  }

  /// Returns the identity matrix.
  ///
  /// Multiplying any matrix or vector by the identity matrix leaves it unchanged,
  /// making it a useful default or starting point for building transforms.
  #[must_use]
  pub fn identity() -> Self {
    Self::new([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  /// Returns a translation matrix.
  ///
  /// Multiplying a world-space vertex by this matrix displaces it by `(x, y, z)`,
  /// effectively moving it to a new position in world space.
  ///
  /// # Arguments
  ///
  /// * `x` - Translation along the x axis
  /// * `y` - Translation along the y axis
  /// * `z` - Translation along the z axis
  #[must_use]
  pub fn translation(x: f32, y: f32, z: f32) -> Self {
    Self::new([
      [1.0, 0.0, 0.0, x],
      [0.0, 1.0, 0.0, y],
      [0.0, 0.0, 1.0, z],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  /// Returns a scale matrix.
  ///
  /// Multiplying a world-space vertex by this matrix scales it independently
  /// along each axis. Passing equal values for `x`, `y`, and `z` produces a
  /// uniform scale that preserves the shape of the geometry.
  ///
  /// # Arguments
  ///
  /// * `x` - Scale factor along the x axis
  /// * `y` - Scale factor along the y axis
  /// * `z` - Scale factor along the z axis
  #[must_use]
  pub fn scale(x: f32, y: f32, z: f32) -> Self {
    Self::new([
      [x, 0.0, 0.0, 0.0],
      [0.0, y, 0.0, 0.0],
      [0.0, 0.0, z, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  /// Returns a rotation matrix around the X axis.
  ///
  /// Rotates geometry in the Y-Z plane. Positive angles rotate counter-clockwise
  /// when viewed from the positive X axis toward the origin, following the
  /// right-hand rule.
  ///
  /// # Arguments
  ///
  /// * `angle` - Rotation angle in radians
  #[must_use]
  pub fn rotation_x(angle: f32) -> Self {
    let (s, c) = angle.sin_cos();
    Self::new([
      [1.0, 0.0, 0.0, 0.0],
      [0.0, c, -s, 0.0],
      [0.0, s, c, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  /// Returns a rotation matrix around the Y axis.
  ///
  /// Rotates geometry in the X-Z plane. Positive angles rotate counter-clockwise
  /// when viewed from the positive Y axis toward the origin, following the
  /// right-hand rule.
  ///
  /// # Arguments
  ///
  /// * `angle` - Rotation angle in radians
  #[must_use]
  pub fn rotation_y(angle: f32) -> Self {
    let (s, c) = angle.sin_cos();
    Self::new([
      [c, 0.0, s, 0.0],
      [0.0, 1.0, 0.0, 0.0],
      [-s, 0.0, c, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  /// Returns a rotation matrix around the Z axis.
  ///
  /// Rotates geometry in the X-Y plane. Positive angles rotate counter-clockwise
  /// when viewed from the positive Z axis toward the origin, following the
  /// right-hand rule.
  ///
  /// # Arguments
  ///
  /// * `angle` - Rotation angle in radians
  #[must_use]
  pub fn rotation_z(angle: f32) -> Self {
    let (s, c) = angle.sin_cos();
    Self::new([
      [c, -s, 0.0, 0.0],
      [s, c, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  /// Returns a look-at view matrix.
  ///
  /// Transforms vertices from world space into camera space, orienting the
  /// camera at `eye` looking toward `target`. This matrix encodes where the
  /// camera is and which direction it is facing, so that subsequent projection
  /// can be applied correctly.
  ///
  /// # Arguments
  ///
  /// * `eye` - The camera position in world space
  /// * `target` - The point the camera is looking at in world space
  /// * `up` - The world up vector, typically `Vec3::new(0.0, 1.0, 0.0)`
  ///
  /// # Panics
  ///
  /// Panics if `eye` and `target` are the same point, or if `up` is zero length,
  /// as both cases produce a degenerate view direction that cannot be normalised.
  #[must_use]
  #[allow(clippy::arithmetic_side_effects)]
  pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
    let dir = target - eye;
    let f = dir.normalise();
    let r = f.cross(up).normalise();
    let u = r.cross(f);

    Self::new([
      [r.x, r.y, r.z, -r.dot(eye)],
      [u.x, u.y, u.z, -u.dot(eye)],
      [-f.x, -f.y, -f.z, f.dot(eye)],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  /// Returns a perspective projection matrix.
  ///
  /// Transforms vertices from camera space into clip space, applying perspective
  /// foreshortening so that objects further from the camera appear smaller. The
  /// resulting clip space coordinates should be passed through a perspective
  /// divide to produce NDC coordinates in the range -1.0 to 1.0.
  ///
  /// # Arguments
  ///
  /// * `fov_y` - Vertical field of view in radians
  /// * `aspect` - Viewport width divided by height
  /// * `near` - Near clipping plane distance; vertices closer than this are not visible
  /// * `far` - Far clipping plane distance; vertices further than this are not visible
  ///
  /// # Panics
  ///
  /// Panics if `aspect` is zero, if `near` and `far` are equal, or if `fov_y`
  /// is zero, as all three cases produce a degenerate projection matrix.
  #[must_use]
  pub fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> Self {
    assert!(
      aspect != 0.0,
      "Mat4::perspective: aspect ratio cannot be zero"
    );
    assert!(
      (near - far).abs() > f32::EPSILON,
      "Mat4::perspective: near and far planes cannot be equal"
    );
    let t = (fov_y / 2.0).tan();
    assert!(t != 0.0, "Mat4::perspective: fov_y cannot be zero");
    let sy = 1.0 / t;
    let sx = sy / aspect;
    let sz = -(far + near) / (far - near);
    let tz = -(2.0 * far * near) / (far - near);
    Self::new([
      [sx, 0.0, 0.0, 0.0],
      [0.0, sy, 0.0, 0.0],
      [0.0, 0.0, sz, tz],
      [0.0, 0.0, -1.0, 0.0],
    ])
  }

  /// Returns a viewport transform matrix that maps NDC coordinates to screen pixels.
  ///
  /// The y-axis is negated to account for the flip between NDC space (y-up,
  /// origin centre) and screen space (y-down, origin top-left). This ensures
  /// that triangles with CCW winding in world space remain CCW after projection
  /// and are not discarded by backface culling.
  ///
  /// # Arguments
  ///
  /// * `x` - Top-left x offset of the viewport in pixels
  /// * `y` - Top-left y offset of the viewport in pixels
  /// * `width` - Viewport width in pixels
  /// * `height` - Viewport height in pixels
  #[must_use]
  pub fn viewport(x: f32, y: f32, width: f32, height: f32) -> Self {
    let hw = width / 2.0;
    let hh = height / 2.0;
    Self::new([
      [hw, 0.0, 0.0, x + hw],
      [0.0, -hh, 0.0, y + hh],
      [0.0, 0.0, 1.0, 0.0],
      [0.0, 0.0, 0.0, 1.0],
    ])
  }

  /// Multiplies `self` by another `Mat4`.
  ///
  /// Matrix multiplication combines two transforms into one. For example,
  /// multiplying a view matrix by a projection matrix produces a single matrix
  /// that applies both transforms in one step. Note that matrix multiplication
  /// is not commutative — the order of operands matters.
  #[must_use]
  // Matrix multiplication requires simultaneous access to two indices (row i, column j)
  // and an inner summation index k. Direct range indexing maps cleanly to the
  // mathematical notation and is clearer than any iterator-based alternative.
  #[allow(clippy::needless_range_loop)]
  pub fn mul_mat4(self, rhs: Self) -> Self {
    let mut result = [[0.0f32; 4]; 4];
    for i in 0..4 {
      for j in 0..4 {
        for k in 0..4 {
          result[i][j] += self.data[i][k] * rhs.data[k][j];
        }
      }
    }
    Self::new(result)
  }

  /// Multiplies `self` by a `Vec4`.
  ///
  /// Applies the matrix transform to a single vertex or direction vector.
  /// This is the core operation of the vertex pipeline — each stage (view,
  /// projection, viewport) is applied by multiplying the matrix by the vertex
  /// as a `Vec4` in homogeneous coordinates.
  ///
  /// # Arguments
  ///
  /// * `rhs` - The `Vec4` to transform
  #[must_use]
  // Vec4 multiplication requires simultaneous row index i and column index j.
  // Range indexing maps directly to the mathematical notation.
  #[allow(clippy::needless_range_loop)]
  pub fn mul_vec4(self, rhs: Vec4) -> Vec4 {
    let v = [rhs.x, rhs.y, rhs.z, rhs.w];
    let mut result = [0.0f32; 4];
    for i in 0..4 {
      for j in 0..4 {
        result[i] += self.data[i][j] * v[j];
      }
    }
    Vec4::new(result[0], result[1], result[2], result[3])
  }
}
