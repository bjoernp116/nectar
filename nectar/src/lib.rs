pub mod logger;
pub mod entity;
pub mod scene;
pub mod archetype;
pub mod window;
pub mod eventloop;
pub mod vertex;
const VERTICES: &[vertex::Vertex] = &[
    vertex::Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    vertex::Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    vertex::Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];
