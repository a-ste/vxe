use luminance::context::GraphicsContext;
use luminance::shader::{UniformInterface};
use luminance::tess::Mode;
use luminance_gl::GL33;
use luminance_glfw::GL33Context;
use luminance::pipeline::{PipelineError, Render};
use crate::data::{Vertex, VertexSemantics, LumProgram, LumTess, LumFrameBuffer, VertexPosition, VertexNormal, VertexRGB};
use crate::vertex;

pub use pipeline::PipelineContext;
pub use render::RenderContext;
pub use tess::TessContext;
pub use luminance::pipeline::PipelineState;
pub use luminance::render_state::RenderState;
use luminance::backend::color_slot::ColorSlot;
use luminance::backend::depth_slot::DepthSlot;
use luminance::texture::{Dimensionable, Sampler};
use luminance::framebuffer::Framebuffer;
use luminance::texture::Dim2;
use crate::context::shader::ShaderContext;

mod render;
mod pipeline;
mod tess;
mod shader;

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

    /// Creates a framebuffer with provided types
    pub fn new_frame_buffer<CS, DS>(&mut self, dim: [i32; 2], mip_size: usize, sampler: Sampler) -> LumFrameBuffer<CS, DS>
        where
            CS: ColorSlot<GL33, Dim2>,
            DS: DepthSlot<GL33, Dim2>
    {
        self.ctx.new_framebuffer::<Dim2, CS, DS>([dim[0] as u32, dim[1] as u32], mip_size, sampler).unwrap()
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
    pub fn new_tess(&mut self, vertices: &Vec<Vertex>, indices: &Vec<u32>) -> LumTess<Vertex, u32> {
        self.ctx.new_tess()
            .set_vertices(vertices.as_slice())
            .set_indices(indices.as_slice())
            .set_mode(Mode::Triangle)
            .build()
            .unwrap()
    }

    /// Creates a quad for rendering frame buffers
    pub fn new_quad(&mut self) -> LumTess<Vertex, u32> {
        self.ctx.new_tess()
            .set_vertices(vec![
                vertex![-1.0, -1.0, 0.0, 1.0, 0.0, 0.0],
                vertex![1.0, -1.0, 0.0, 0.0, 1.0, 0.0],
                vertex![1.0, 1.0, 0.0, 0.0, 0.0, 1.0],
                vertex![-1.0, 1.0, 0.0, 1.0, 1.0, 1.0],
            ])
            .set_indices(vec![0, 2, 1, 0, 3, 2])
            .set_mode(Mode::Triangle)
            .build()
            .unwrap()
    }

    /// Creates a pipeline and runs the closure
    pub fn pipeline<D, CS, DS, F>(&mut self, buffer: &Framebuffer<GL33, D, CS, DS>, state: PipelineState, func: F) -> Render<PipelineError>
        where
            F: FnOnce(PipelineContext, ShaderContext) -> Result<(), PipelineError>,
            D: Dimensionable,
            CS: ColorSlot<GL33, D>,
            DS: DepthSlot<GL33, D>,
    {
        self.ctx.new_pipeline_gate()
            .pipeline(buffer, &state, |pipeline, shd_gate| {
                func(PipelineContext::new(pipeline), ShaderContext::new(shd_gate))
            }).assume()
    }

    /// Retrieves back buffer for rendering onto screen
    pub fn back_buffer(&mut self) -> LumFrameBuffer<(), ()> {
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

    pub fn resolution(&self) -> [i32; 2] {
        let res = self.ctx.window.get_size();
        [res.0, res.1]
    }
}


