mod bindings;
mod model;
mod renderer;
mod resources;
mod textures;
mod window;
use tokio;
mod camera;
#[tokio::main]
async fn main() {
    let engine = window::Engine::new().await;
    engine.run();
}
