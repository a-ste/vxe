use luminance::shading_gate::ShadingGate;
use luminance_gl::GL33;
use luminance::pipeline::{Pipeline, PipelineError};
use crate::context::{RenderContext};
use luminance::shader::UniformInterface;
use crate::data::LumProgram;

/// Pipeline context for binding shaders
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
    pub fn use_shader<I, F>(&mut self, shader: &mut LumProgram<I>, func: F) -> Result<(), PipelineError>
        where
            F: FnOnce(RenderContext, &I) -> Result<(), PipelineError>,
            I: UniformInterface<GL33>
    {
        self.shd_gate.shade(shader, |iface, uni, rdr_gate| {
            func(RenderContext::new(iface, rdr_gate), uni)
        })
    }
}