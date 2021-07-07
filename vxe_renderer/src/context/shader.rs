use luminance::shading_gate::ShadingGate;
use luminance_gl::GL33;
use crate::context::RenderContext;
use luminance::pipeline::PipelineError;
use crate::data::LumProgram;
use luminance::shader::UniformInterface;

/// Pipeline context for binding shaders
#[allow(dead_code)]
pub struct ShaderContext<'a> {
    shd_gate: ShadingGate<'a, GL33>
}

#[allow(dead_code)]
impl ShaderContext<'_> {
    pub(crate) fn new(shd_gate: ShadingGate<GL33>) -> ShaderContext {
        ShaderContext {
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