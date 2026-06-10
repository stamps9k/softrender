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
  #[must_use]
  pub fn new(x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  /// Returns the dot product of `self` and `rhs`.
  #[must_use]
  pub fn dot(self, rhs: Self) -> f32 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
  }

  /// Returns the cross product of `self` and `rhs`.
  #[must_use]
  pub fn cross(self, rhs: Self) -> Self {
    Self {
      x: self.y * rhs.z - self.z * rhs.y,
      y: self.z * rhs.x - self.x * rhs.z,
      z: self.x * rhs.y - self.y * rhs.x,
    }
  }

  /// Returns the length (magnitude) of the vector.
  #[must_use]
  pub fn length(self) -> f32 {
    self.dot(self).sqrt()
  }

  /// Returns a normalised (unit length) copy of the vector.
  ///
  /// # Panics
  ///
  /// Panics if the vector has zero length.
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
