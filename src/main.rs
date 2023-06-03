use crate::lib::vertex::Vertex;
use crate::lib::window;

#[path = "lib/mod.rs"]
mod lib;

pub fn main() {
    let triangle_height: f32 = (std::f32::consts::PI / 6.0).cos();

    pollster::block_on(window::open(
        include_str!("shaders/shader.wgsl"),
        &[
            Vertex {
                position: [0.0, 1.0, 0.0, 1.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-triangle_height, -0.5, 0.0, 1.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [triangle_height, -0.5, 0.0, 1.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ],
    ));
}
