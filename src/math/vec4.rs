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
  ///
  /// # Arguments
  ///
  /// * `x` - The x component
  /// * `y` - The y component
  /// * `z` - The z component
  /// * `w` - The w component, typically `1.0` for positions and `0.0` for directions
  #[must_use]
  pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
    Self { x, y, z, w }
  }

  /// Creates a `Vec4` from a `Vec3` and a `w` component.
  ///
  /// Promotes a 3D vector to homogeneous coordinates for matrix multiplication.
  /// Pass `w = 1.0` for positions so that translation is applied, or `w = 0.0`
  /// for direction vectors so that translation is ignored.
  ///
  /// # Arguments
  ///
  /// * `v` - The `Vec3` to promote
  /// * `w` - The w component, typically `1.0` for positions and `0.0` for
  ///   directions
  #[must_use]
  pub fn from_vec3(v: Vec3, w: f32) -> Self {
    Self::new(v.x, v.y, v.z, w)
  }

  /// Creates a `Vec4` from a `Vec3` and a `w` component.
  ///
  /// Demotes a 4D vector to back to 3D coordinates for converting back to down
  /// to screen space.
  /// Simple dropping of the w component.
  #[must_use]
  pub fn to_vec3(self) -> Vec3 {
    Vec3::new(self.x, self.y, self.z)
  }

  /// Returns the dot product of `self` and `rhs`.
  ///
  /// Computes the sum of the products of each corresponding component,
  /// including the `w` component. For most geometric uses, prefer
  /// [`Vec3::dot`] unless you specifically need a 4D dot product.
  ///
  /// # Arguments
  ///
  /// * `rhs` - The right-hand side vector
  #[must_use]
  pub fn dot(self, rhs: Self) -> f32 {
    self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
  }

  /// Performs the perspective divide, returning a `Vec3` in NDC space.
  ///
  /// Divides the `x`, `y`, and `z` components by `w`, converting from
  /// homogeneous clip space coordinates to NDC coordinates in the range
  /// -1.0 to 1.0. This step is what produces perspective foreshortening —
  /// vertices further from the camera have a larger `w`, so dividing by it
  /// makes them appear smaller on screen.
  ///
  /// # Panics
  ///
  /// Panics if `w` is zero, as division by zero produces no meaningful result
  /// and indicates a vertex at or behind the camera origin.
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
