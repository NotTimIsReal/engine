use std::f32::consts::FRAC_PI_2;
use std::time::Duration;

use cgmath::{self, Matrix, SquareMatrix, Vector3};
use cgmath::{InnerSpace, Rad};
use winit::dpi::PhysicalPosition;
use winit::event::MouseScrollDelta;
use winit::keyboard::{self, Key, ModifiersState};
#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);
const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;
pub struct Camera {
    pub position: cgmath::Point3<f32>,
    yaw: cgmath::Rad<f32>,
    pitch: cgmath::Rad<f32>,
}
impl Camera {
    pub fn new<
        P: Into<cgmath::Point3<f32>>,
        Y: Into<cgmath::Rad<f32>>,
        I: Into<cgmath::Rad<f32>>,
    >(
        position: P,
        yaw: Y,
        pitch: I,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }
    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();
        cgmath::Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        )
    }
}
pub struct Projection {
    aspect: f32,
    fovy: Rad<f32>,
    znear: f32,
    zfar: f32,
}
impl Projection {
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }
    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }
    pub fn calc_matrix(&self) -> cgmath::Matrix4<f32> {
        //OPENGL_TO_WGPU_MATRIX *
        cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view: [[f32; 4]; 4],
    view_proj: [[f32; 4]; 4],
    inv_proj: [[f32; 4]; 4],
    inv_view: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_position: [0.0; 4],
            view: cgmath::Matrix4::identity().into(),
            view_proj: cgmath::Matrix4::identity().into(),
            inv_proj: cgmath::Matrix4::identity().into(),
            inv_view: cgmath::Matrix4::identity().into(),
        }
    }
    pub fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        let proj = projection.calc_matrix();
        let view = camera.calc_matrix();
        let view_proj = proj * view;
        self.view = view.into();
        self.view_proj = view_proj.into();
        self.inv_proj = proj.invert().unwrap().into();
        self.inv_view = view.transpose().into();
    }
}
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
    Jump,
    Crouch,
}
#[derive(Debug)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    scroll: f32,
    speed: f32,
    sensitivity: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            scroll: 0.0,
            speed,
            sensitivity,
        }
    }
    pub fn process_keyboard(&mut self, movement: CameraMovement) -> bool {
        //reset the movement
        match movement {
            CameraMovement::Forward => {
                self.amount_forward = 1.0;
                true
            }
            CameraMovement::Backward => {
                self.amount_backward = 1.0;
                true
            }
            CameraMovement::Left => {
                self.amount_left = 1.0;
                true
            }
            CameraMovement::Right => {
                self.amount_right = 1.0;
                true
            }
            CameraMovement::Jump => {
                self.amount_up = 1.0;
                true
            }
            CameraMovement::Crouch => {
                self.amount_down = 1.0;
                true
            }
            _ => false,
        }
    }
    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self.rotate_horizontal = mouse_dx as f32;
        self.rotate_vertical = mouse_dy as f32;
    }
    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        self.scroll = -match delta {
            MouseScrollDelta::LineDelta(_, scroll) => -scroll * 0.5,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => -*scroll as f32,
        };
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: Duration) {
        let dt = dt.as_secs_f32();
        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        camera.position += (self.amount_forward - self.amount_backward) * self.speed * dt * forward;
        camera.position += (self.amount_right - self.amount_left) * self.speed * dt * right;

        let (pitch_sin, pitch_cos) = camera.pitch.0.sin_cos();
        let scrollward =
            Vector3::new(pitch_cos * yaw_cos, pitch_sin, pitch_cos * yaw_sin).normalize();
        camera.position += scrollward * self.scroll * self.speed * self.sensitivity * dt;
        self.scroll = 0.0;
        camera.position.y += (self.amount_up - self.amount_down) * self.speed * dt;
        // camera.position.y += (self.amount_up - self.amount_down) * self.speed * 1000.0 * dt;
        // let yaw = Rad(self.rotate_horizontal * self.sensitivity / 100.0);
        // let pitch = Rad(-self.rotate_vertical * self.sensitivity / 100.0);
        // camera.yaw = if yaw == Rad(0.0) { camera.yaw } else { yaw };
        // camera.pitch = if pitch == Rad(-0.0) {
        //     camera.pitch
        // } else {
        //     pitch
        // };
        camera.yaw += Rad(self.rotate_horizontal * self.sensitivity * dt);
        camera.pitch += Rad(-self.rotate_vertical * self.sensitivity * dt);

        // if self.rotate_vertical != 0.0 && self.rotate_horizontal != 0.0 {
        //     println!(
        //         "yaw and pitch: {:#?} {:#?}",
        //         Rad(self.rotate_horizontal) * self.sensitivity * dt,
        //         Rad(-self.rotate_vertical) * self.sensitivity * dt
        //     );
        // }

        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        if camera.pitch < -Rad(SAFE_FRAC_PI_2) {
            camera.pitch = -Rad(SAFE_FRAC_PI_2);
        } else if camera.pitch > Rad(SAFE_FRAC_PI_2) {
            camera.pitch = Rad(SAFE_FRAC_PI_2);
        }
        // reset amount
        self.amount_left = 0.0;
        self.amount_right = 0.0;
        self.amount_forward = 0.0;
        self.amount_backward = 0.0;
        self.amount_up = 0.0;
        self.amount_down = 0.0;
    }
}
