use crate::math::{Mat4, Vec3, Vec4};
use crate::render::ScreenVertex;

/// A camera that manages the view and projection transforms for the render pipeline.
pub struct Camera {
  eye: Vec3,
  target: Vec3,
  up: Vec3,
  fov_y: f32,
  aspect: f32,
  near: f32,
  far: f32,
  view: Mat4,
  projection: Mat4,
}

impl Camera {
  /// Creates a new `Camera` and caches the view and projection matrices.
  ///
  /// # Arguments
  ///
  /// * `eye` - Camera position in world space
  /// * `target` - The point the camera is looking at
  /// * `up` - The world up vector, typically `Vec3::new(0.0, 1.0, 0.0)`
  /// * `fov_y` - Vertical field of view in radians
  /// * `aspect` - Viewport width divided by height
  /// * `near` - Near clipping plane distance
  /// * `far` - Far clipping plane distance
  #[must_use]
  pub fn new(
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    fov_y: f32,
    aspect: f32,
    near: f32,
    far: f32,
  ) -> Self {
    let view = Mat4::look_at(eye, target, up);
    let projection = Mat4::perspective(fov_y, aspect, near, far);
    Self {
      eye,
      target,
      up,
      fov_y,
      aspect,
      near,
      far,
      view,
      projection,
    }
  }

  /// Returns the cached view matrix.
  ///
  /// The view matrix transforms vertices from world space into camera space.
  /// It is recomputed automatically when the camera position or target changes
  /// via [`Camera::set_eye`] or [`Camera::set_target`].
  #[must_use]
  pub fn view_matrix(&self) -> Mat4 {
    self.view
  }

  /// Returns the cached projection matrix.
  ///
  /// The projection matrix transforms vertices from camera space into clip space,
  /// applying perspective foreshortening so that objects further from the camera
  /// appear smaller. It is recomputed automatically when the field of view or
  /// aspect ratio changes via [`Camera::set_fov_y`] or [`Camera::set_aspect`].  
  #[must_use]
  pub fn projection_matrix(&self) -> Mat4 {
    self.projection
  }

  /// Returns a viewport matrix mapping NDC coordinates to screen pixels.
  ///
  /// The y-axis is negated to account for the flip between NDC space (y-up,
  /// origin centre) and screen space (y-down, origin top-left). This ensures
  /// that triangles with CCW winding in world space remain CCW after projection
  /// and are not discarded by backface culling.
  ///
  /// # Arguments
  ///
  /// * `width` - Viewport width in pixels
  /// * `height` - Viewport height in pixels
  #[must_use]
  pub fn viewport_matrix(&self, width: f32, height: f32) -> Mat4 {
    Mat4::viewport(0.0, 0.0, width, height)
  }

  /// Projects a single world-space vertex through the full pipeline,
  /// returning a `ScreenVertex` ready for the rasterizer.
  ///
  /// Pipeline stages:
  /// - World space → camera space via the view matrix
  /// - Camera space → clip space via the projection matrix
  /// - Clip space → NDC via the perspective divide
  /// - NDC → screen space via the viewport matrix
  ///
  /// For batch projection of multiple vertices, use `project_mesh` (upcoming).
  ///
  /// # Arguments
  ///
  /// * `v` - A vertex in world space
  /// * `screen_width` - Viewport width in pixels
  /// * `screen_height` - Viewport height in pixels
  #[must_use]
  pub fn project_vertex(
    &self,
    v: Vec3,
    screen_width: f32,
    screen_height: f32,
  ) -> ScreenVertex {
    let view_space = self.view.mul_vec4(Vec4::from_vec3(v, 1.0));
    let clip_space = self.projection.mul_vec4(view_space);

    let ndc = clip_space.perspective_divide();
    let ndc_flipped = Vec3::new(ndc.x, -ndc.y, ndc.z);

    let viewport = self.viewport_matrix(screen_width, screen_height);
    let screen = viewport.mul_vec4(Vec4::from_vec3(ndc_flipped, 1.0));
    #[allow(clippy::cast_possible_truncation)]
    ScreenVertex::new(screen.x as i32, screen.y as i32, screen.z)
  }

  /// Updates the camera position and recomputes the view matrix.
  ///
  /// The cached view matrix is updated immediately so subsequent calls to
  /// [`Camera::project_vertex`] will reflect the new position.
  ///
  /// # Arguments
  ///
  /// * `eye` - The new camera position in world space
  pub fn set_eye(&mut self, eye: Vec3) {
    self.eye = eye;
    self.recompute_view();
  }

  /// Updates the camera target and recomputes the view matrix.
  ///
  /// The cached view matrix is updated immediately so subsequent calls to
  /// [`Camera::project_vertex`] will reflect the new target.
  ///
  /// # Arguments
  ///
  /// * `target` - The new target position in world space
  pub fn set_target(&mut self, target: Vec3) {
    self.target = target;
    self.recompute_view();
  }

  /// Updates the vertical field of view and recomputes the projection matrix.
  ///
  /// The cached projection matrix is updated immediately so subsequent calls to
  /// [`Camera::project_vertex`] will reflect the new field of view.
  ///
  /// # Arguments
  ///
  /// * `fov_y` - The new vertical field of view in radians
  pub fn set_fov_y(&mut self, fov_y: f32) {
    self.fov_y = fov_y;
    self.recompute_projection();
  }

  /// Updates the aspect ratio and recomputes the projection matrix.
  ///
  /// The cached projection matrix is updated immediately so subsequent calls to
  /// [`Camera::project_vertex`] will reflect the new aspect ratio. Typically
  /// called when the viewport is resized.
  ///
  /// # Arguments
  ///
  /// * `aspect` - The new aspect ratio, expressed as viewport width divided by height
  pub fn set_aspect(&mut self, aspect: f32) {
    self.aspect = aspect;
    self.recompute_projection();
  }

  /// Recomputes and caches the view matrix from the current eye, target, and up vectors.
  fn recompute_view(&mut self) {
    self.view = Mat4::look_at(self.eye, self.target, self.up);
  }

  /// Recomputes and caches the projection matrix from the current field of view, aspect ratio, near, and far planes.
  fn recompute_projection(&mut self) {
    self.projection =
      Mat4::perspective(self.fov_y, self.aspect, self.near, self.far);
  }
}
