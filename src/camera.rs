use cgmath::InnerSpace;
use cgmath::{self, Vector3};
#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}
impl Camera {
    fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    view_position: [f32; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
            view_position: [0.0; 4],
        }
    }
    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
        self.view_position = camera.eye.to_homogeneous().into();
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}
pub struct CameraController {
    speed: f32,
    position: cgmath::Vector3<f32>,
    movement: Vec<CameraMovement>,
}
impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            movement: Vec::new(),
        }
    }
    pub fn process_keyboard(&mut self, direction: CameraMovement) {
        self.movement.push(direction);
    }
    pub fn update_camera(&mut self, camera: &mut Camera) {
        use cgmath::EuclideanSpace;
        let forward: Vector3<f32> = camera.target.to_vec() - camera.eye.to_vec();
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();
        if self.movement.contains(&CameraMovement::Forward) && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.movement.contains(&CameraMovement::Backward) {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();
        if self.movement.contains(&CameraMovement::Right) {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.movement.contains(&CameraMovement::Left) {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
        self.movement = Vec::new();
    }
}
