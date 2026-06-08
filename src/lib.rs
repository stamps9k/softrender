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
