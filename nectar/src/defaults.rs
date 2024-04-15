#![allow(dead_code)]
#![allow(unused_imports)]
use crate::Vertex;
use crate::component;

#[derive(Debug, Clone, Copy)]
pub struct Transform3D {
    pub position: (i32, i32, i32),
    pub rotation: (i32, i32, i32),
    pub size:     (i32, i32, i32),
}
component!(Transform3D);
impl Transform3D {
    pub fn new()->Transform3D {
        Transform3D {
            position: (0,0,0),
            rotation: (0,0,0),
            size: (1,1,1)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Renderer {
    pub vertecies: Vec<Vertex>,

}
component!(Renderer);
