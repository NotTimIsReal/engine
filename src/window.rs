use std::{rc::Rc, sync::Arc};

use crate::{renderer::Renderer, Plugin};
use winit::{
    event::{DeviceEvent, ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    keyboard::Key,
    platform::{
        modifier_supplement::KeyEventExtModifierSupplement, run_on_demand::EventLoopExtRunOnDemand,
    },
    window::{CursorGrabMode, Fullscreen, WindowBuilder},
};

pub struct Engine<'a> {
    renderer: Renderer<'a>,
    event_loop: EventLoop<()>,
    plugins: Vec<Box<dyn Plugin>>,
    window: Arc<winit::window::Window>,
}
impl<'a> Engine<'a> {
    pub async fn new() -> Engine<'a> {
        let event_loop = EventLoop::new().unwrap();
        let window = WindowBuilder::new()
            .with_title("Game")
            .build(&event_loop)
            .unwrap();
        window.focus_window();

        window.set_cursor_visible(false);
        #[cfg(target_os = "macos")]
        window
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .unwrap();
        #[cfg(not(target_os = "macos"))]
        window.set_cursor_grab(CursorGrabMode::Confined).unwrap();
        let window = Arc::new(window);
        let renderer = Renderer::new(window.clone()).await;
        let plugins = vec![];
        Engine {
            renderer,
            event_loop,
            plugins,
            window: window,
        }
        //depending on the platform, use vulkan,opengl or metal
    }
    //TODO: plugins
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
    pub fn run(mut self) {
        for plugin in self.plugins.iter() {
            plugin.init();
        }

        self.event_loop
            .run(move |event, elwt| {
                let time_now = std::time::Instant::now();
                let mainevent = &event;

                self.window
                    .set_fullscreen(Some(Fullscreen::Borderless(self.window.current_monitor())));

                match mainevent {
                    Event::DeviceEvent { event, .. } => match event {
                        DeviceEvent::MouseMotion { delta } => {
                            self.renderer
                                .camera_controller
                                .process_mouse(delta.0 as f64, delta.1 as f64);
                        }
                        _ => {}
                    },
                    Event::WindowEvent { window_id, event } => {
                        match event {
                            // WindowEvent::MouseInput { .. } => {
                            //     if !in_window {
                            //         self.renderer.cursor_grab();
                            //         in_window = true;
                            //     }
                            // }
                            WindowEvent::KeyboardInput { event, .. } => {
                                if event.state == ElementState::Pressed {
                                    match event.key_without_modifiers().as_ref() {
                                        Key::Character("w") => {
                                            self.renderer.camera_controller.process_keyboard(
                                                crate::camera::CameraMovement::Forward,
                                            );
                                        }
                                        Key::Character("a") => {
                                            self.renderer.camera_controller.process_keyboard(
                                                crate::camera::CameraMovement::Left,
                                            );
                                        }
                                        Key::Character("s") => {
                                            self.renderer.camera_controller.process_keyboard(
                                                crate::camera::CameraMovement::Backward,
                                            );
                                        }
                                        Key::Character("d") => {
                                            self.renderer.camera_controller.process_keyboard(
                                                crate::camera::CameraMovement::Right,
                                            );
                                        }
                                        Key::Named(winit::keyboard::NamedKey::Space) => {
                                            self.renderer.camera_controller.process_keyboard(
                                                crate::camera::CameraMovement::Jump,
                                            );
                                        }
                                        Key::Named(winit::keyboard::NamedKey::Shift) => {
                                            self.renderer.camera_controller.process_keyboard(
                                                crate::camera::CameraMovement::Crouch,
                                            );
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            WindowEvent::Resized(physical_size) => {
                                self.renderer.resize(physical_size.clone());
                            }
                            WindowEvent::CloseRequested => {
                                elwt.exit();
                            }

                            // WindowEvent::CursorMoved { position, .. } => {
                            //     self.renderer
                            //         .camera_controller
                            //         .process_mouse(position.x as f64, position.y as f64);
                            // }
                            WindowEvent::MouseWheel { delta, .. } => {
                                self.renderer.camera_controller.process_scroll(&delta);
                            }
                            //redraw requested
                            _ => {}
                        }
                    }
                    _ => {}
                }
                //spawn code below in a different thread
                self.renderer.update(time_now.elapsed()).unwrap();
                self.renderer.render();

                let avg_fps: f64 = 1.0 / (time_now.elapsed().as_secs_f64());
            })
            .unwrap();
        //cleanup
    }
}
