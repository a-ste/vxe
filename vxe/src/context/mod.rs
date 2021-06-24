use luminance_glfw::GL33Context;
use crate::data::Vertex;
use luminance::context::GraphicsContext;
use luminance::tess::{Mode, Interleaved, Tess};

pub struct Context {
    ctx: GL33Context
}

impl Context {
    pub fn new(ctx: GL33Context) -> Context {
        Context {
            ctx
        }
    }

    pub fn vertex_set(&mut self, vertices: Vec<Vertex>) -> Tess<GL33, Vertex, (), (), Interleaved> {
        self.ctx.new_tess()
            .set_vertices(vertices.as_slice())
            .set_mode(Mode::Triangle)
            .build()
            .unwrap()
    }
}