use crate::camera::Camera;
use crate::geometry::Mesh;
use crate::math::{Mat4, Vec3, Vec4};
use crate::render::{rasterizer::fill_triangle, Framebuffer, ZBuffer};

/// Orchestrates the full render pipeline for a scene.
///
/// Holds all per-frame state (framebuffer, depth buffer, camera, meshes)
/// and drives the vertex pipeline and rasterizer each frame.
pub struct Renderer {
  framebuffer: Framebuffer,
  zbuffer: ZBuffer,
  camera: Camera,
  meshes: Vec<Mesh>,
  width: usize,
  height: usize,
}

impl Renderer {
  /// Colour palette used to distinguish triangle faces during rendering.
  /// Pairs of triangles share a colour, giving each cube face a distinct
  /// solid colour. Will be superseded by texture and lighting in milestone 7.
  const PALETTE: [[u8; 3]; 6] = [
    [255, 100, 100], // red
    [100, 255, 100], // green
    [100, 100, 255], // blue
    [255, 255, 100], // yellow
    [255, 100, 255], // magenta
    [100, 255, 255], // cyan
  ];

  /// Creates a new [`Renderer`] with a default camera and a single cube mesh.
  ///
  /// # Errors
  ///
  /// Returns an error if the framebuffer dimensions overflow `usize`.
  ///
  /// # Arguments
  ///
  /// * `width` - Viewport width in pixels
  /// * `height` - Viewport height in pixels
  pub fn new(width: usize, height: usize) -> Result<Self, String> {
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_precision_loss)]
    let aspect = width as f32 / height as f32;

    let camera = Camera::new(
      Vec3::new(0.0, 0.0, 3.0),
      Vec3::new(0.0, 0.0, 0.0),
      Vec3::new(0.0, 1.0, 0.0),
      std::f32::consts::FRAC_PI_3,
      aspect,
      0.1,
      100.0,
    );

    Ok(Self {
      framebuffer: Framebuffer::new(width, height)?,
      zbuffer: ZBuffer::new(width, height),
      camera,
      meshes: vec![Mesh::cube()],
      width,
      height,
    })
  }

  /// Renders a single frame at the given timestamp and returns the resulting
  /// RGBA pixel data.
  ///
  /// The timestamp drives the rotation of all meshes in the scene.
  /// The returned slice is valid until the next call to `render_frame`.
  ///
  /// # Arguments
  ///
  /// * `timestamp_ms` - Elapsed time in milliseconds, typically from
  ///   `requestAnimationFrame`
  #[must_use]
  pub fn render_frame(&mut self, timestamp_ms: f64) -> &[u8] {
    self.framebuffer.clear(0, 0, 0);
    self.zbuffer.clear();

    #[allow(clippy::cast_possible_truncation)]
    let angle = (timestamp_ms as f32 / 1000.0) * std::f32::consts::FRAC_PI_3;
    #[allow(clippy::cast_precision_loss)]
    let width = self.width as f32;
    #[allow(clippy::cast_precision_loss)]
    let height = self.height as f32;

    for mesh in &self.meshes {
      let transform = Mat4::rotation_y(angle).mul_mat4(mesh.transform);

      for (i, triangle) in mesh.triangles.iter().enumerate() {
        #[allow(clippy::arithmetic_side_effects)]
        // modulo by PALETTE.len() which is always non-zero
        let color = Self::PALETTE[i % Self::PALETTE.len()];

        let sv0 = self.camera.project_vertex(
          transform
            .mul_vec4(Vec4::from_vec3(triangle[0].position, 1.0))
            .to_vec3(),
          width,
          height,
        );
        let sv1 = self.camera.project_vertex(
          transform
            .mul_vec4(Vec4::from_vec3(triangle[1].position, 1.0))
            .to_vec3(),
          width,
          height,
        );
        let sv2 = self.camera.project_vertex(
          transform
            .mul_vec4(Vec4::from_vec3(triangle[2].position, 1.0))
            .to_vec3(),
          width,
          height,
        );

        fill_triangle(
          &mut self.framebuffer,
          &mut self.zbuffer,
          sv0,
          sv1,
          sv2,
          color,
        );
      }
    }

    self.framebuffer.as_rgba()
  }
}
