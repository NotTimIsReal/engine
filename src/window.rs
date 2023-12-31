use sdl2::{self, video::Window};

use crate::renderer::Renderer;

pub struct Engine {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    //canvas: &'a sdl2::render::Canvas<sdl2::video::Window>,
    renderer: Renderer,
}
impl Engine {
    pub async fn new() -> Engine {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        // Set the hint based on the platform
        let mut windowbuilder = video_subsystem.window("Game", 800, 600);

        #[cfg(target_os = "macos")]
        windowbuilder.metal_view();
        #[cfg(target_os = "windows")]
        windowbuilder.vulkan();
        #[cfg(target_os = "linux")]
        windowbuilder.opengl();
        let window = windowbuilder
            .fullscreen_desktop()
            .position_centered()
            .build()
            .unwrap();

        #[cfg(target_os = "macos")]
        sdl2::hint::set("SDL_RENDER_DRIVER", "metal");

        #[cfg(target_os = "windows")]
        sdl2::hint::set("SDL_RENDER_DRIVER", "opengl");

        #[cfg(target_os = "linux")]
        sdl2::hint::set("SDL_RENDER_DRIVER", "vulkan");
        let renderer = Renderer::new(window).await;
        // let canvas = &window.into_canvas().build().unwrap();

        Engine {
            sdl_context,
            video_subsystem,
            // canvas,
            renderer,
        }
        //depending on the platform, use vulkan,opengl or metal
    }
    pub fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    sdl2::event::Event::KeyDown { keycode, .. } => match keycode {
                        Some(sdl2::keyboard::Keycode::W) => {
                            self.renderer
                                .camera_controller
                                .process_keyboard(crate::camera::CameraMovement::Forward);
                        }
                        Some(sdl2::keyboard::Keycode::A) => {
                            self.renderer
                                .camera_controller
                                .process_keyboard(crate::camera::CameraMovement::Left);
                        }
                        Some(sdl2::keyboard::Keycode::S) => {
                            self.renderer
                                .camera_controller
                                .process_keyboard(crate::camera::CameraMovement::Backward);
                        }
                        Some(sdl2::keyboard::Keycode::D) => {
                            self.renderer
                                .camera_controller
                                .process_keyboard(crate::camera::CameraMovement::Right);
                        }
                        _ => {}
                    },
                    //redraw requested
                    _ => {}
                }

                self.renderer.update();
                self.renderer.render();
            }
        }
        //cleanup
    }
}
