use crate::Framebuffer;
use crate::ScreenVertex;
use crate::ZBuffer;

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
  zb: &mut ZBuffer,
  v0: ScreenVertex,
  v1: ScreenVertex,
  v2: ScreenVertex,
  color: [u8; 3],
) {
  // Compute the area of the triangle (doubled) using the edge function.
  // If zero, the triangle is degenerate (collinear points); skip it.
  // If negative, the triangle is clockwise; skip it (backface cull).
  let area = edge_function(v0.x, v0.y, v1.x, v1.y, v2.x, v2.y);
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
  let max_x = v0.x.max(v1.x).max(v2.x).min(fb_width);
  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  let max_y = v0.y.max(v1.y).max(v2.y).min(fb_height);
  let min_x = v0.x.min(v1.x).min(v2.x).max(0);
  let min_y = v0.y.min(v1.y).min(v2.y).max(0);

  // Iterate over every pixel in the bounding box.
  for py in min_y..=max_y {
    for px in min_x..=max_x {
      // Evaluate the edge function for each edge at this pixel.
      #[allow(clippy::cast_precision_loss)]
      let w0 =
        edge_function(v1.x, v1.y, v2.x, v2.y, px, py) as f32 / area as f32;
      #[allow(clippy::cast_precision_loss)]
      let w1 =
        edge_function(v2.x, v2.y, v0.x, v0.y, px, py) as f32 / area as f32;
      #[allow(clippy::cast_precision_loss)]
      let w2 =
        edge_function(v0.x, v0.y, v1.x, v1.y, px, py) as f32 / area as f32;

      // The pixel is inside the triangle if all three are non-negative.
      if w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0 {
        #[allow(clippy::cast_sign_loss)]
        let z = w0 * v0.z + w1 * v1.z + w2 * v2.z;

        // Pixel is inside triangle, so coordinates must be non-negative and fit in i32 so casting to usize is safe.
        #[allow(clippy::cast_sign_loss)]
        if zb.test_and_set(px as usize, py as usize, z) {
          fb.set_pixel(px as usize, py as usize, color[0], color[1], color[2]);
        }
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
      &mut ZBuffer::new(10, 10),
      ScreenVertex::new(0, 0, 0.0),
      ScreenVertex::new(9, 0, 0.0),
      ScreenVertex::new(4, 9, 0.0),
      [255, 0, 0],
    );
    // The centroid (~4, 3) should be red.
    assert_eq!(fb.unwrap().get_pixel(4, 3), (255, 0, 0));
  }

  #[test]
  fn test_fill_triangle_corner_pixel_outside() {
    let mut fb = Framebuffer::new(10, 10);
    let mut zb = ZBuffer::new(10, 10);
    fill_triangle(
      &mut fb.as_mut().unwrap(),
      &mut zb,
      ScreenVertex::new(0, 0, 0.0),
      ScreenVertex::new(9, 0, 0.0),
      ScreenVertex::new(4, 9, 0.0),
      [255, 0, 0],
    );
    // Bottom-right corner should be untouched.
    assert_eq!(fb.unwrap().get_pixel(9, 9), (0, 0, 0));
  }

  #[test]
  fn test_fill_triangle_degenerate_skipped() {
    let mut fb = Framebuffer::new(10, 10);
    let mut zb = ZBuffer::new(10, 10);
    // Three collinear points — should draw nothing.
    fill_triangle(
      &mut fb.as_mut().unwrap(),
      &mut zb,
      ScreenVertex::new(0, 0, 0.0),
      ScreenVertex::new(5, 0, 0.0),
      ScreenVertex::new(9, 0, 0.0),
      [255, 0, 0],
    );
    assert_eq!(fb.unwrap().get_pixel(5, 0), (0, 0, 0));
  }

  #[test]
  fn test_fill_triangle_clockwise_skipped() {
    let mut fb = Framebuffer::new(10, 10);
    let mut zb = ZBuffer::new(10, 10);
    // Same triangle as the first test but with vertices in clockwise order.
    fill_triangle(
      &mut fb.as_mut().unwrap(),
      &mut zb,
      ScreenVertex::new(4, 9, 0.0),
      ScreenVertex::new(9, 0, 0.0),
      ScreenVertex::new(0, 0, 0.0),
      [255, 0, 0],
    );
    assert_eq!(fb.unwrap().get_pixel(4, 3), (0, 0, 0));
  }

  #[test]
  fn test_fill_triangle_zbuffer() {
    let mut fb = Framebuffer::new(10, 10);
    let mut zb = ZBuffer::new(10, 10);
    // Two overlapping triangles with different depths.
    fill_triangle(
      &mut fb.as_mut().unwrap(),
      &mut zb,
      ScreenVertex::new(0, 0, 0.5),
      ScreenVertex::new(9, 0, 0.5),
      ScreenVertex::new(4, 9, 0.5),
      [255, 0, 0],
    );
    fill_triangle(
      &mut fb.as_mut().unwrap(),
      &mut zb,
      ScreenVertex::new(0, 0, 0.0),
      ScreenVertex::new(9, 0, 0.0),
      ScreenVertex::new(4, 9, 0.0),
      [0, 255, 0],
    );
    // The second triangle is closer and should overwrite the first.
    assert_eq!(fb.unwrap().get_pixel(4, 3), (0, 255, 0));
  }
}
