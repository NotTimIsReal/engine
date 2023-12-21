use sdl2;

struct Engine {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    window: sdl2::video::Window,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Engine {
    fn new() -> Engine {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        // Set the hint based on the platform
        #[cfg(target_os = "macos")]
        sdl2::hint::set("SDL_RENDER_DRIVER", "metal");

        #[cfg(target_os = "windows")]
        sdl2::hint::set("SDL_RENDER_DRIVER", "opengl");

        #[cfg(target_os = "linux")]
        sdl2::hint::set("SDL_RENDER_DRIVER", "vulkan");
        let window = video_subsystem
            .window("Game", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        Engine {
            sdl_context,
            video_subsystem,
            window,
            canvas,
        }
        //depending on the platform, use vulkan,opengl or metal
    }
    fn run(&mut self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }
        }
    }
}
