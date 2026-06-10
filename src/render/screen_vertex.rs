/// A vertex projected into screen space, ready for rasterization.
///
/// `x` and `y` are integer pixel coordinates; `z` is the interpolated
/// depth value used for the z-buffer test.
#[derive(Clone, Copy)]
pub struct ScreenVertex {
  pub x: i32,
  pub y: i32,
  pub z: f32,
}

impl ScreenVertex {
  /// Create a new [`ScreenVertex`].
  #[must_use]
  pub fn new(x: i32, y: i32, z: f32) -> Self {
    Self { x, y, z }
  }
}
