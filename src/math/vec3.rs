use std::ops::{Add, Mul, Neg, Sub};

/// A 3-component vector of `f32` values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Vec3 {
  /// Creates a new `Vec3`.
  ///
  /// # Arguments
  ///
  /// * `x` - The x component
  /// * `y` - The y component
  /// * `z` - The z component
  #[must_use]
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  /// Returns the dot product of `self` and `rhs`.
  ///
  /// The dot product is a scalar value that represents the cosine of the angle
  /// between two vectors scaled by their lengths. A result of zero means the
  /// vectors are perpendicular, positive means they point in roughly the same
  /// direction, and negative means they point in roughly opposite directions.
  ///
  /// # Arguments
  ///
  /// * `rhs` - The right-hand side vector
  #[must_use]
  pub fn dot(self, rhs: Self) -> f32 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }

  /// Returns the cross product of `self` and `rhs`.
  ///
  /// The cross product produces a new vector perpendicular to both `self` and
  /// `rhs`, following the right-hand rule. The magnitude of the result equals
  /// the area of the parallelogram formed by the two vectors. Used in the
  /// look-at matrix to compute the camera's right and up basis vectors.
  ///
  /// # Arguments
  ///
  /// * `rhs` - The right-hand side vector
  #[must_use]
  pub fn cross(self, rhs: Self) -> Self {
    Self {
      x: self.y * rhs.z - self.z * rhs.y,
      y: self.z * rhs.x - self.x * rhs.z,
      z: self.x * rhs.y - self.y * rhs.x,
    }
  }

  /// Returns the length (magnitude) of the vector.
  ///
  /// Computed as the square root of the sum of the squared components.
  /// If only relative lengths need comparing, consider avoiding the square
  /// root by comparing squared lengths directly for better performance.
  #[must_use]
  pub fn length(self) -> f32 {
    self.dot(self).sqrt()
  }

  /// Returns a normalised (unit length) copy of the vector.
  ///
  /// A normalised vector has a length of 1.0 and preserves only the direction
  /// of the original vector. Normalisation is required before using a vector
  /// as a basis direction, such as when computing the camera axes in
  /// [`Mat4::look_at`].
  ///
  /// # Panics
  ///
  /// Panics if the vector has zero length, as a zero vector has no direction
  /// and cannot be normalised.
  #[must_use]
  pub fn normalise(self) -> Self {
    let len = self.length();
    assert!(len != 0.0, "Vec3::normalise called on a zero-length vector");
    Self::new(self.x / len, self.y / len, self.z / len)
  }
}

impl Add for Vec3 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl Sub for Vec3 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl Mul<f32> for Vec3 {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self {
    Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
  }
}

impl Neg for Vec3 {
  type Output = Self;
  fn neg(self) -> Self {
    Self::new(-self.x, -self.y, -self.z)
  }
}
