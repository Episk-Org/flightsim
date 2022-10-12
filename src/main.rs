use window_handler::WindowHandler;

mod asset_manager;
mod camera;
mod entity;
mod helper;
mod input_handler;
mod mesh;
mod mesh_factory;
mod model;
mod plane;
mod water;
mod window_handler;
mod world;

fn main() {
    println!("Installing keylogger...");
    println!("Keylogger installed successfully!");
    WindowHandler::new(1600, 900, false).run();
}
