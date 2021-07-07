use vxe_renderer::data::{Vertex};
use vxe_renderer::context::Context;
use crate::shader::Material;

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    materials: Option<Box<dyn Material>>,
}

impl Mesh
{
    pub fn new<'a, V, I>(vertices: V, indices: I) -> Mesh
        where
            V: Into<Vec<Vertex>>,
            I: Into<Vec<u32>>,
    {
        Mesh {
            vertices: vertices.into(),
            indices: indices.into(),
            materials: None,
        }
    }

    pub(crate) fn create(ctx: &mut Context) {

    }
}