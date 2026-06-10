pub struct ZBuffer {
  data: Vec<f32>,
  width: usize,
  /// Retained for self-documentation and future use (e.g. clearing, resizing).
  #[allow(dead_code)]
  height: usize,
}

impl ZBuffer {
  /// Create a new [`ZBuffer`] with the given dimensions, initialised to `f32::MAX`.
  ///
  /// # Panics
  ///
  /// Panics if `width * height` overflows `usize`, as this indicates a logic
  /// error in the caller as width and height should have been validated when creating the framebuffer.
  #[must_use]
  #[allow(clippy::panic)]
  pub fn new(width: usize, height: usize) -> Self {
    let size = width
      .checked_mul(height)
      .unwrap_or_else(|| {
        panic!("ZBuffer::new overflow: {width} * {height} overflows usize, suggests a logic error in the caller")
      });
    Self {
      data: vec![f32::INFINITY; size],
      width,
      height,
    }
  }

  /// Returns the depth value at `(x, y)`.
  ///
  /// # Panics
  ///
  /// Panics if `(x, y)` is out of bounds or if `y * width + x` overflows `usize`,
  /// as either indicates a logic error in the caller.
  #[must_use]
  #[allow(clippy::panic)]
  pub fn get_depth(&self, x: usize, y: usize) -> f32 {
    let index = y
			.checked_mul(self.width)
			.and_then(|i| i.checked_add(x))
			.unwrap_or_else(|| {
				panic!("ZBuffer index overflow: ({x}, {y}) suggests a logic error in the caller")
			});
    self.data.get(index).copied().unwrap_or_else(|| {
    	panic!("ZBuffer::get_depth out of bounds: ({x}, {y}) suggests a logic error in the caller")
  	})
  }

  /// Set the depth value at `(x, y)`.
  ///
  /// # Panics
  ///
  /// Panics if `(x, y)` is out of bounds or if `y * width + x` overflows `usize`,
  /// as either indicates a logic error in the caller.
  #[allow(clippy::panic)]
  pub fn set_depth(&mut self, x: usize, y: usize, depth: f32) {
    let index = y
			.checked_mul(self.width)
			.and_then(|i| i.checked_add(x))
			.unwrap_or_else(|| {
				panic!("ZBuffer::set_depth overflow: ({x}, {y}) suggests a logic error in the caller")
			});
    *self.data.get_mut(index).unwrap_or_else(|| {
			panic!("ZBuffer::set_depth out of bounds: ({x}, {y}) suggests a logic error in the caller")
		}) = depth;
  }

  /// Test whether `z` is closer than the current depth at `(x, y)`.
  /// If so, update the buffer and return `true`. Otherwise return `false`.
  ///
  /// # Panics
  ///
  /// Panics if `(x, y)` is out of bounds or if `y * width + x` overflows `usize`,
  /// as either indicates a logic error in the caller.
  pub fn test_and_set(&mut self, x: usize, y: usize, z: f32) -> bool {
    if z < self.get_depth(x, y) {
      self.set_depth(x, y, z);
      true
    } else {
      false
    }
  }
}
