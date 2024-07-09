mod bindings;
mod hdr;
mod model;
mod renderer;
mod resources;
mod text;
mod textures;
mod window;
use tokio;
mod camera;
use env_logger;
pub trait Plugin {
    fn init(&self);
    fn update(&self);
    fn render(&self);
}
#[tokio::main]
async fn main() {
    env_logger::init();
    let engine = window::Engine::new().await;

    engine.run();
}
