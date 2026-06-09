use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

pub struct Framebuffer {
  pub width: usize,
  pub height: usize,
  data: Vec<u8>, // Packed RGB triples: [R, G, B, R, G, B, ...]
}

impl Framebuffer {
  /// Create a new framebuffer filled with black.
  ///
  /// # Errors
  ///
  /// Returns an error if `width * height * 3` overflows `usize`.
  pub fn new(width: usize, height: usize) -> Result<Self, String> {
    let len = width
      .checked_mul(height)
      .and_then(|n| n.checked_mul(3))
      .ok_or_else(|| {
        format!("Framebuffer dimensions {width}x{height} overflow usize")
      })?;
    Ok(Self {
      width,
      height,
      data: vec![0u8; len],
    })
  }

  /// Set a pixel at (x, y) to the given RGB colour.
  /// (0, 0) is the top-left corner.
  /// Does nothing if the coordinates are out of bounds.
  ///
  /// # Panics
  ///
  /// Panics if internal index arithmetic overflows, which should be
  /// impossible if the framebuffer was constructed successfully.
  #[allow(clippy::many_single_char_names)]
  #[allow(clippy::panic)]
  pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
    if x >= self.width || y >= self.height {
      return;
    }
    // Safety: x < self.width and y < self.height, and the buffer was
    // successfully allocated in new(), so this arithmetic cannot overflow.
    let Some(index) = y
      .checked_mul(self.width)
      .and_then(|n| n.checked_add(x))
      .and_then(|n| n.checked_mul(3))
    else {
      panic!("index arithmetic overflowed despite bounds check — this is a bug")
    };

    let (Some(index1), Some(index2)) =
      (index.checked_add(1), index.checked_add(2))
    else {
      panic!("index arithmetic overflowed despite bounds check — this is a bug")
    };

    self.data[index] = r;
    self.data[index1] = g;
    self.data[index2] = b;
  }

  /// Read the RGB colour at (x, y).
  /// Returns (0, 0, 0) if out of bounds.
  ///
  /// # Panics
  ///
  /// Panics if internal index arithmetic overflows, which should be
  /// impossible if the framebuffer was constructed successfully.
  #[allow(clippy::many_single_char_names)]
  #[must_use]
  #[allow(clippy::panic)]
  pub fn get_pixel(&self, x: usize, y: usize) -> (u8, u8, u8) {
    if x >= self.width || y >= self.height {
      return (0, 0, 0);
    }

    // Safety: x < self.width and y < self.height, and the buffer was
    // successfully allocated in new(), so this arithmetic cannot overflow.
    let Some(index) = y
      .checked_mul(self.width)
      .and_then(|n| n.checked_add(x))
      .and_then(|n| n.checked_mul(3))
    else {
      panic!("index arithmetic overflowed despite bounds check — this is a bug")
    };

    let (Some(index1), Some(index2)) =
      (index.checked_add(1), index.checked_add(2))
    else {
      panic!("index arithmetic overflowed despite bounds check — this is a bug")
    };

    (self.data[index], self.data[index1], self.data[index2])
  }

  /// Fill the entire framebuffer with a single colour.
  #[allow(clippy::many_single_char_names)]
  #[allow(clippy::panic)]
  pub fn clear(&mut self, r: u8, g: u8, b: u8) {
    for chunk in self.data.chunks_exact_mut(3) {
      chunk[0] = r;
      chunk[1] = g;
      chunk[2] = b;
    }
  }

  /// Write the framebuffer to a binary PPM file (P6 format).
  ///
  /// PPM is a trivial image format — no external crates needed.
  /// Open the output with any image viewer, or convert with `ImageMagick`:
  ///   convert output.ppm output.png
  ///
  /// # Errors
  ///
  /// Returns an error if the file cannot be created or written to.
  pub fn save_ppm(&self, path: &Path) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    // PPM header: magic number, dimensions, max channel value
    writeln!(writer, "P6")?;
    writeln!(writer, "{} {}", self.width, self.height)?;
    writeln!(writer, "255")?;

    // Raw RGB bytes — no separators needed in P6 format
    writer.write_all(&self.data)?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::path::PathBuf;

  #[test]
  fn test_dimensions() {
    let fb = Framebuffer::new(800, 600);
    assert_eq!(fb.width, 800);
    assert_eq!(fb.height, 600);
    assert_eq!(fb.data.len(), 800 * 600 * 3);
  }

  #[test]
  fn test_set_and_get_pixel() {
    let mut fb = Framebuffer::new(10, 10);
    fb.set_pixel(3, 7, 255, 128, 0);
    assert_eq!(fb.get_pixel(3, 7), (255, 128, 0));
  }

  #[test]
  fn test_out_of_bounds_does_not_panic() {
    let mut fb = Framebuffer::new(10, 10);
    fb.set_pixel(10, 10, 255, 0, 0); // exactly at boundary — should be ignored
    fb.set_pixel(100, 100, 255, 0, 0); // way out — should be ignored
  }

  #[test]
  fn test_clear() {
    let mut fb = Framebuffer::new(4, 4);
    fb.clear(100, 150, 200);
    for y in 0..4 {
      for x in 0..4 {
        assert_eq!(fb.get_pixel(x, y), (100, 150, 200));
      }
    }
  }

  #[test]
  fn test_save_ppm() {
    let mut fb = Framebuffer::new(64, 64);

    // Paint a simple gradient so the file is visually non-trivial
    for y in 0..64 {
      for x in 0..64 {
        fb.set_pixel(x, y, (x * 4) as u8, (y * 4) as u8, 128);
      }
    }

    let path = PathBuf::from("/tmp/test_output.ppm");
    fb.save_ppm(&path).expect("Failed to write PPM");
    assert!(path.exists());
  }
}
