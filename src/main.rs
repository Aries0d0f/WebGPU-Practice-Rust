use crate::lib::vertex::Vertex;
use crate::lib::window;

#[path = "lib/mod.rs"]
mod lib;

pub fn main() {
    pollster::block_on(window::open(
        include_str!("shaders/shader.wgsl"),
        &[
            Vertex {
                position: [0.0, 0.5, 0.0, 1.0],
                color: [1.0, 0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0, 1.0],
                color: [0.0, 1.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0, 1.0],
                color: [0.0, 0.0, 1.0, 1.0],
            },
        ],
    ));
}
