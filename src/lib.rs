pub mod camera;
pub mod geometry;
pub mod math;
pub mod render;
pub mod renderer;

use camera::Camera;
use math::Vec3;
use render::rasterizer::fill_triangle;
use render::Framebuffer;
use render::ZBuffer;
use renderer::Renderer;

use js_sys::Uint8Array;
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
/// # Arguments
///
/// * `width` - The width of the frame in pixels
/// * `height` - The height of the frame in pixels
///
/// # Returns
///
/// A `Vec<u8>` containing RGBA pixel data of the rendered scene, where each
/// pixel is represented by 4 bytes (R, G, B, A).
///
/// # Errors
///
/// Returns `Err` if the framebuffer cannot be created with the given
/// dimensions.
#[wasm_bindgen]
pub fn render_test_triangle(
  width: usize,
  height: usize,
) -> Result<Vec<u8>, JsValue> {
  #[allow(clippy::cast_precision_loss)]
  let (screen_width, screen_height) = (width as f32, height as f32);

  let camera = Camera::new(
    Vec3::new(0.0, 0.0, 3.0), // eye: sitting 3 units back on the z axis
    Vec3::new(0.0, 0.0, 0.0), // target: looking at the origin
    Vec3::new(0.0, 1.0, 0.0), // up: y is up
    std::f32::consts::FRAC_PI_4, // fov_y: 45 degrees
    screen_width / screen_height, // aspect
    0.1,                      // near
    100.0,                    // far
  );

  let mut fb =
    Framebuffer::new(width, height).map_err(|e| JsValue::from_str(&e))?;
  let mut zb = ZBuffer::new(width, height);

  let v0 = camera.project_vertex(
    Vec3::new(-0.75, 0.5, 0.0),
    screen_width,
    screen_height,
  );
  let v1 = camera.project_vertex(
    Vec3::new(-0.25, -0.5, 0.0),
    screen_width,
    screen_height,
  );
  let v2 = camera.project_vertex(
    Vec3::new(0.25, 0.5, 0.0),
    screen_width,
    screen_height,
  );
  fill_triangle(&mut fb, &mut zb, v0, v1, v2, [255, 0, 0]);

  let v3 = camera.project_vertex(
    Vec3::new(-0.25, 0.5, -0.5),
    screen_width,
    screen_height,
  );
  let v4 = camera.project_vertex(
    Vec3::new(0.25, -0.5, -0.5),
    screen_width,
    screen_height,
  );
  let v5 = camera.project_vertex(
    Vec3::new(0.75, 0.5, -0.5),
    screen_width,
    screen_height,
  );
  fill_triangle(&mut fb, &mut zb, v3, v4, v5, [0, 255, 0]);

  Ok(fb.as_rgba().to_vec())
}

/// A WASM-exposed renderer that maintains scene and pipeline state across
/// frames.
///
/// Construct once with [`WasmRenderer::new`], then call
/// [`WasmRenderer::render_frame`] each animation tick.
#[wasm_bindgen]
pub struct WasmRenderer {
  inner: Renderer,
}

#[wasm_bindgen]
impl WasmRenderer {
  /// Creates a new [`WasmRenderer`] with the given viewport dimensions.
  ///
  /// # Arguments
  ///
  /// * `width` - Viewport width in pixels
  /// * `height` - Viewport height in pixels
  ///
  /// # Errors
  ///
  /// Returns `Err` if the framebuffer dimensions overflow `usize`.
  #[wasm_bindgen(constructor)]
  pub fn new(width: usize, height: usize) -> Result<WasmRenderer, JsValue> {
    let inner =
      Renderer::new(width, height).map_err(|e| JsValue::from_str(&e))?;
    Ok(Self { inner })
  }

  /// Renders a single frame and returns the RGBA pixel data as a
  /// [`Uint8Array`] suitable for use with the browser canvas `putImageData`
  /// API.
  ///
  /// # Arguments
  ///
  /// * `timestamp_ms` - Elapsed time in milliseconds, typically from
  ///   `requestAnimationFrame`
  pub fn render_frame(&mut self, timestamp_ms: f64) -> Uint8Array {
    Uint8Array::from(self.inner.render_frame(timestamp_ms))
  }
}
