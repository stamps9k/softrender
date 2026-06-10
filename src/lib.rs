mod framebuffer;
pub use framebuffer::Framebuffer;
mod rasterizer;
pub use rasterizer::fill_triangle;
mod zbuffer;
pub use zbuffer::ZBuffer;
mod screen_vertex;
pub use screen_vertex::ScreenVertex;

use wasm_bindgen::prelude::*;

/// A short test function to ensure that lib and wasm are working correctly.
///
/// # Arguments
///
/// * `width` - The width of the frame in pixels
/// * `height` - The height of the frame in pixels
///
/// # Returns
///
/// A `Vec<u8>` containing RGBA pixel data, with every pixel set to zero.
///
/// # Errors
///
/// Returns `Err` if `width * height * 4` overflows a `u32`.
#[wasm_bindgen]
pub fn solid_frame(width: u32, height: u32) -> Result<Vec<u8>, String> {
  let size = width
    .checked_mul(height)
    .and_then(|n| n.checked_mul(4))
    .ok_or_else(|| String::from("buffer size overflowed"))?;
  let mut buffer = vec![0u8; size as usize];

  for pixel in buffer.chunks_mut(4) {
    pixel[0] = 255; // R
    pixel[1] = 0; // G
    pixel[2] = 128; // B
    pixel[3] = 255; // A
  }

  Ok(buffer)
}

/// Render a test triangle to a framebuffer and return the result as RGBA.
///
/// # Errors
///
/// Returns `Err` if the framebuffer cannot be created with the given dimensions.
#[wasm_bindgen]
pub fn render_test_triangle(
  width: usize,
  height: usize,
) -> Result<Vec<u8>, JsValue> {
  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  let width_i32 = (width as i32).checked_sub(10).ok_or_else(|| {
    JsValue::from_str(
      "render_test_triangle: width too small to compute triangle vertices",
    )
  })?;
  #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
  let height_i32 = (height as i32).checked_sub(10).ok_or_else(|| {
    JsValue::from_str(
      "render_test_triangle: height too small to compute triangle vertices",
    )
  })?;
  #[allow(
    clippy::integer_division,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap
  )]
  let mid_x = (width as i32) / 2;

  let mut fb: Framebuffer =
    Framebuffer::new(width, height).map_err(|e| JsValue::from_str(&e))?;
  let mut zb = ZBuffer::new(width, height);

  #[allow(clippy::integer_division)]
  // The triangle vertices are painting a test pattern, so truncation is acceptable and intentional.
  fill_triangle(
    &mut fb,
    &mut zb,
    ScreenVertex::new(10, 10, -1.0), //Top Left
    ScreenVertex::new(width_i32 / 2, 10, 1.0), //Top Right
    ScreenVertex::new(mid_x, height_i32, 1.0), //Bottom Middle
    [255, 0, 0],
  );

  #[allow(clippy::integer_division)]
  // The triangle vertices are painting a test pattern, so truncation is acceptable and intentional.
  fill_triangle(
    &mut fb,
    &mut zb,
    ScreenVertex::new(width_i32 / 3, 10, 0.0), //Top Left
    ScreenVertex::new(width_i32, 10, 0.0),     //Top Right
    ScreenVertex::new(mid_x, height_i32, 0.0), //Bottom Middle
    [0, 255, 0],
  );
  Ok(fb.into_rgba())
}
