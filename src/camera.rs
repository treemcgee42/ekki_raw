#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

struct Camera {
    look_from: cgmath::Point3<f32>,
    look_at: cgmath::Point3<f32>,
    up_direction: cgmath::Vector3<f32>,
    aspect_ratio: f32,
    vertical_fov: f32,
    z_near: f32, // distance to near clipping plane
    z_far: f32,  // distance to far clipping plane
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        // to camera space
        let view = cgmath::Matrix4::look_at_rh(self.look_from, self.look_at, self.up_direction);
        // add perspective
        let proj = cgmath::perspective(
            cgmath::Deg(self.vertical_fov),
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        );

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
