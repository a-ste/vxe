use luminance::context::GraphicsContext;
use luminance::shader::Program;
use luminance::tess::{Interleaved, Mode, Tess};
use luminance::framebuffer::Framebuffer;
use luminance_gl::GL33;
use luminance::texture::Dim2;
use luminance_glfw::GL33Context;
use luminance::pipeline::{PipelineError, PipelineState, Render};
use crate::data::{Vertex, VertexSemantics};

mod render;
mod pipeline;
mod tess;

/// Backend type for Shader program
pub type LumProgram = Program<GL33, VertexSemantics, (), ()>;

/// Backend type for Tesselation, a set of vertices or pretty much all the mesh data that will be sent to GPU
pub type LumTess = Tess<GL33, Vertex, (), (), Interleaved>;

/// Backend type for Frame Buffers
pub type LumFrameBuffer = Framebuffer<GL33, Dim2, (), ()>;

/// Struct that will be passed to Handler's functions
#[allow(dead_code)]
pub struct Context<'a> {
    ctx: &'a mut GL33Context,
    fps: u32,
}

#[allow(dead_code)]
impl Context<'_> {
    pub(crate) fn new(ctx: &mut GL33Context, fps: u32) -> Context {
        Context {
            ctx,
            fps
        }
    }

    /// Loads shader code and creates a program
    pub fn new_shader_program(&mut self, vertex: &str, fragment: &str) -> LumProgram {
        self.ctx.new_shader_program::<VertexSemantics, (), ()>()
            .from_strings(vertex, None, None, fragment)
            .unwrap()
            .ignore_warnings()
    }

    /// Creates a new tesselation
    pub fn new_tess(&mut self, vertices: &Vec<Vertex>) -> LumTess {
        self.ctx.new_tess()
            .set_vertices(vertices.as_slice())
            .set_mode(Mode::Triangle)
            .build()
            .unwrap()
    }

    /// Creates a pipeline and runs the closure
    pub fn pipeline<F>(&mut self, buffer: &LumFrameBuffer, func: F) -> Render<PipelineError>
        where
            F: FnOnce(PipelineContext) -> Result<(), PipelineError>
    {
        self.ctx.new_pipeline_gate()
            .pipeline(buffer, &PipelineState::default(), |pipeline, shd_gate| {
                func(PipelineContext::new(pipeline, shd_gate))
            }).assume()
    }

    /// Retrieves back buffer for rendering onto screen
    pub fn back_buffer(&mut self) -> LumFrameBuffer {
        self.ctx.back_buffer().unwrap()
    }

    /// Gets FPS
    pub fn get_fps(&self) -> u32 {
        self.fps
    }
}

pub use pipeline::PipelineContext;
pub use render::RenderContext;
pub use tess::TessContext;
