use crate::renderer::Renderer;
use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::Key,
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{Fullscreen, WindowBuilder},
};

pub struct Engine {
    renderer: Renderer,
    event_loop: EventLoop<()>,
}
impl Engine {
    pub async fn new() -> Engine {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title("Game")
            .build(&event_loop)
            .unwrap();
        window.set_fullscreen(Some(Fullscreen::Borderless(window.current_monitor())));
        let renderer = Renderer::new(window).await;
        Engine {
            renderer,
            event_loop,
        }
        //depending on the platform, use vulkan,opengl or metal
    }
    pub fn run(mut self) {
        self.event_loop
            .run(move |event, elwt| {
                let time_now = std::time::Instant::now();
                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::KeyboardInput { event, .. } => {
                            if event.state == ElementState::Pressed {
                                match event.key_without_modifiers().as_ref() {
                                    Key::Character("w") => {
                                        self.renderer.camera_controller.process_keyboard(
                                            crate::camera::CameraMovement::Forward,
                                        );
                                    }
                                    Key::Character("a") => {
                                        self.renderer
                                            .camera_controller
                                            .process_keyboard(crate::camera::CameraMovement::Left);
                                    }
                                    Key::Character("s") => {
                                        self.renderer.camera_controller.process_keyboard(
                                            crate::camera::CameraMovement::Backward,
                                        );
                                    }
                                    Key::Character("d") => {
                                        self.renderer
                                            .camera_controller
                                            .process_keyboard(crate::camera::CameraMovement::Right);
                                    }
                                    _ => {}
                                }
                            }
                        }
                        WindowEvent::Resized(physical_size) => {
                            self.renderer.resize(physical_size);
                        }
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }

                        //redraw requested
                        _ => {}
                    }
                }
                self.renderer.update();
                self.renderer.render();
                // std::thread::sleep(std::time::Duration::from_millis(16 / 2));
                let time_elapsed = time_now.elapsed();
                let avg_fps: f64 = 1.0 / (time_elapsed.as_secs_f64());
                //only print when build config is debug
                #[cfg(debug_assertions)]
                println!("FPS: {}", avg_fps.round());
            })
            .unwrap();
        //cleanup
    }
}
