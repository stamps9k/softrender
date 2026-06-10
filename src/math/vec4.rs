use std::ops::{Add, Mul, Neg, Sub};

use crate::math::Vec3;

/// A 4-component homogeneous vector of `f32` values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec4 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
}

impl Vec4 {
  /// Creates a new `Vec4`.
  #[must_use]
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }

  /// Creates a `Vec4` from a `Vec3` and a `w` component.
  #[must_use]
  pub fn from_vec3(v: Vec3, w: f32) -> Self {
    Self::new(v.x, v.y, v.z, w)
  }

  /// Returns the dot product of `self` and `rhs`.
  #[must_use]
  pub fn dot(self, rhs: Self) -> f32 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
  }

  /// Performs the perspective divide, returning a `Vec3` in NDC space.
  ///
  /// # Panics
  ///
  /// Panics if `w` is zero.
  #[must_use]
  pub fn perspective_divide(self) -> Vec3 {
    assert!(self.w != 0.0, "Vec4::perspective_divide called with w = 0");
    Vec3::new(self.x / self.w, self.y / self.w, self.z / self.w)
  }
}

impl Add for Vec4 {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    Self::new(
      self.x + rhs.x,
      self.y + rhs.y,
      self.z + rhs.z,
      self.w + rhs.w,
    )
  }
}

impl Sub for Vec4 {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    Self::new(
      self.x - rhs.x,
      self.y - rhs.y,
      self.z - rhs.z,
      self.w - rhs.w,
    )
  }
}

impl Mul<f32> for Vec4 {
  type Output = Self;
  fn mul(self, rhs: f32) -> Self {
    Self::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
  }
}

impl Neg for Vec4 {
  type Output = Self;
  fn neg(self) -> Self {
    Self::new(-self.x, -self.y, -self.z, -self.w)
  }
}
