use luminance_glfw::GL33Context;
use crate::data::Vertex;
use luminance::context::GraphicsContext;
use luminance_gl::GL33;
use luminance::tess::{Mode, Interleaved, Tess};

#[allow(dead_code)]
pub struct Context {
    ctx: GL33Context
}

#[allow(dead_code)]
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