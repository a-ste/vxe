use luminance::shading_gate::ShadingGate;
use luminance_gl::GL33;
use luminance::pipeline::{Pipeline, PipelineError};
use crate::context::{LumProgram, RenderContext};

/// Struct that will be passed whenever you create a pipeline, has useful functions like binding shaders
#[allow(dead_code)]
pub struct PipelineContext<'a> {
    pipeline: Pipeline<'a, GL33>,
    shd_gate: ShadingGate<'a, GL33>
}

#[allow(dead_code)]
impl PipelineContext<'_> {
    pub(crate) fn new<'a>(pipeline: Pipeline<'a, GL33>, shd_gate: ShadingGate<'a, GL33>) -> PipelineContext<'a> {
        PipelineContext {
            pipeline,
            shd_gate
        }
    }

    /// Binds specified shader and runs the closure
    pub fn use_shader<F>(&mut self, shader: &mut LumProgram, func: F) -> Result<(), PipelineError>
        where
            F: FnOnce(RenderContext) -> Result<(), PipelineError>
    {
        self.shd_gate.shade(shader, |_, _, rdr_gate| {
            func(RenderContext::new(rdr_gate))
        })
    }
}