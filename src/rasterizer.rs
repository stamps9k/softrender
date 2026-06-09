use crate::Framebuffer;

/// Compute the edge function for two vertices `a`, `b` and a point `p`.
///
/// Returns a positive value if `p` is to the left of the edge a→b,
/// negative if to the right, and zero if exactly on the edge.
#[allow(clippy::panic)]
fn edge_function(ax: i32, ay: i32, bx: i32, by: i32, px: i32, py: i32) -> i32 {
  let bx_ax = (bx).checked_sub(ax).unwrap_or_else(|| {
    panic!("edge_function overflow: coordinate values suggest a logic error in the caller")
  });
  let py_ay = (py).checked_sub(ay).unwrap_or_else(|| {
    panic!("edge_function overflow: coordinate values suggest a logic error in the caller")
  });
  let by_ay = (by).checked_sub(ay).unwrap_or_else(|| {
    panic!("edge_function overflow: coordinate values suggest a logic error in the caller")
  });
  let px_ax = (px).checked_sub(ax).unwrap_or_else(|| {
    panic!("edge_function overflow: coordinate values suggest a logic error in the caller")
  });
  let a = bx_ax.checked_mul(py_ay).unwrap_or_else(|| {
    panic!("edge_function overflow: coordinate values suggest a logic error in the caller")
  });
  let b = by_ay.checked_mul(px_ax).unwrap_or_else(|| {
    panic!("edge_function overflow: coordinate values suggest a logic error in the caller")
  });
  a.checked_sub(b).unwrap_or_else(|| {
    panic!("edge_function overflow: coordinate values suggest a logic error in the caller")
  })
}

/// Fill a triangle defined by three vertices with a solid colour.
///
/// Vertices are given as `(x, y)` integer pixel coordinates.
/// Winding order is counter-clockwise; clockwise triangles are silently skipped
/// (backface culled).
///
/// # Panics
///
/// Does not panic. Out-of-bounds pixels are clipped by [`Framebuffer::set_pixel`].
pub fn fill_triangle(
  fb: &mut Framebuffer,
  v0: (i32, i32),
  v1: (i32, i32),
  v2: (i32, i32),
  color: [u8; 3],
) {
  let (x0, y0) = v0;
  let (x1, y1) = v1;
  let (x2, y2) = v2;

  // Compute the area of the triangle (doubled) using the edge function.
  // If zero, the triangle is degenerate (collinear points); skip it.
  // If negative, the triangle is clockwise; skip it (backface cull).
  let area = edge_function(x0, y0, x1, y1, x2, y2);
  if area <= 0 {
    return;
  }

  // Compute the axis-aligned bounding box, clamped to the framebuffer.
  #[allow(clippy::panic)]
  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  let fb_width = (fb.width as i32).checked_sub(1).unwrap_or_else(|| {
    panic!("framebuffer width underflow: suggests a logic error in the caller")
  });
  #[allow(clippy::panic)]
  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  let fb_height = (fb.height as i32).checked_sub(1).unwrap_or_else(|| {
    panic!("framebuffer height underflow: suggests a logic error in the caller")
  });

  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  let max_x = x0.max(x1).max(x2).min(fb_width);
  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  let max_y = y0.max(y1).max(y2).min(fb_height);
  let min_x = x0.min(x1).min(x2).max(0);
  let min_y = y0.min(y1).min(y2).max(0);

  // Iterate over every pixel in the bounding box.
  for py in min_y..=max_y {
    for px in min_x..=max_x {
      // Evaluate the edge function for each edge at this pixel.
      let w0 = edge_function(x1, y1, x2, y2, px, py);
      let w1 = edge_function(x2, y2, x0, y0, px, py);
      let w2 = edge_function(x0, y0, x1, y1, px, py);

      // The pixel is inside the triangle if all three are non-negative.
      if w0 >= 0 && w1 >= 0 && w2 >= 0 {
        #[allow(clippy::cast_sign_loss)]
        fb.set_pixel(px as usize, py as usize, color[0], color[1], color[2]);
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fill_triangle_center_pixel_inside() {
    let mut fb = Framebuffer::new(10, 10);
    // A large triangle that covers the center of the framebuffer.
    fill_triangle(
      &mut fb.as_mut().unwrap(),
      (0, 0),
      (9, 0),
      (4, 9),
      [255, 0, 0],
    );
    // The centroid (~4, 3) should be red.
    assert_eq!(fb.unwrap().get_pixel(4, 3), (255, 0, 0));
  }

  #[test]
  fn test_fill_triangle_corner_pixel_outside() {
    let mut fb = Framebuffer::new(10, 10);
    fill_triangle(
      &mut fb.as_mut().unwrap(),
      (0, 0),
      (9, 0),
      (4, 9),
      [255, 0, 0],
    );
    // Bottom-right corner should be untouched.
    assert_eq!(fb.unwrap().get_pixel(9, 9), (0, 0, 0));
  }

  #[test]
  fn test_fill_triangle_degenerate_skipped() {
    let mut fb = Framebuffer::new(10, 10);
    // Three collinear points — should draw nothing.
    fill_triangle(
      &mut fb.as_mut().unwrap(),
      (0, 0),
      (5, 0),
      (9, 0),
      [255, 0, 0],
    );
    assert_eq!(fb.unwrap().get_pixel(5, 0), (0, 0, 0));
  }

  #[test]
  fn test_fill_triangle_clockwise_skipped() {
    let mut fb = Framebuffer::new(10, 10);
    // Same triangle as the first test but with vertices in clockwise order.
    fill_triangle(
      &mut fb.as_mut().unwrap(),
      (4, 9),
      (9, 0),
      (0, 0),
      [255, 0, 0],
    );
    assert_eq!(fb.unwrap().get_pixel(4, 3), (0, 0, 0));
  }
}
