mod bindings;
mod model;
mod renderer;
mod resources;
mod textures;
mod window;
use tokio;
mod camera;
pub trait Plugin{
    fn init(&self);
    fn update(&self);
    fn render(&self);

}
#[tokio::main]
async fn main() {
    let engine = window::Engine::new().await;
    engine.run();
}
