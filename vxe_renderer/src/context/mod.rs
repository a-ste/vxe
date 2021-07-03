use luminance::context::GraphicsContext;
use luminance::shader::{UniformInterface};
use luminance::tess::Mode;
use luminance_gl::GL33;
use luminance_glfw::GL33Context;
use luminance::pipeline::{PipelineError, Render};
use crate::data::{Vertex, VertexSemantics, LumProgram, LumTess, LumFrameBuffer};

pub use pipeline::PipelineContext;
pub use render::RenderContext;
pub use tess::TessContext;
pub use luminance::pipeline::PipelineState;
pub use luminance::render_state::RenderState;

mod render;
mod pipeline;
mod tess;

/// Context for creating things and rendering
#[allow(dead_code)]
pub struct Context<'a> {
    ctx: &'a mut GL33Context,
    fps: u32,
    delta: f32,
}

#[allow(dead_code)]
impl Context<'_> {
    pub(crate) fn new(ctx: &mut GL33Context, fps: u32, delta: f32) -> Context {
        Context {
            ctx,
            fps,
            delta
        }
    }

    /// Loads shader code and creates a program
    pub fn new_shader_program<I>(&mut self, vertex: &str, fragment: &str) -> LumProgram<I>
        where
            I: UniformInterface<GL33>
    {
        self.ctx.new_shader_program::<VertexSemantics, (), I>()
            .from_strings(vertex, None, None, fragment)
            .unwrap()
            .ignore_warnings()
    }

    /// Creates a new tesselation
    pub fn new_tess(&mut self, vertices: &Vec<Vertex>, indices: &Vec<u32>) -> LumTess {
        self.ctx.new_tess()
            .set_vertices(vertices.as_slice())
            .set_indices(indices.as_slice())
            .set_mode(Mode::Triangle)
            .build()
            .unwrap()
    }

    /// Creates a pipeline and runs the closure
    pub fn pipeline<F>(&mut self, buffer: &LumFrameBuffer, state: PipelineState, func: F) -> Render<PipelineError>
        where
            F: FnOnce(PipelineContext) -> Result<(), PipelineError>
    {
        self.ctx.new_pipeline_gate()
            .pipeline(buffer, &state, |pipeline, shd_gate| {
                func(PipelineContext::new(pipeline, shd_gate))
            }).assume()
    }

    /// Retrieves back buffer for rendering onto screen
    pub fn back_buffer(&mut self) -> LumFrameBuffer {
        self.ctx.back_buffer().unwrap()
    }

    /// Gets FPS
    pub fn fps(&self) -> u32 {
        self.fps
    }

    /// Gets delta time between frames
    pub fn delta(&self) -> f32 {
        self.delta
    }
}


