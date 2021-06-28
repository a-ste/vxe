use luminance_gl::GL33;
use luminance::render_gate::RenderGate;
use luminance::pipeline::PipelineError;
use crate::context::tess::TessContext;
use luminance::render_state::RenderState;
use luminance::shader::{ProgramInterface};
use crate::data::shader::Uniform;
use luminance::backend::shader::Uniformable;

/// Render context for drawing tessellations
#[allow(dead_code)]
pub struct RenderContext<'a> {
    pgr_interface: ProgramInterface<'a, GL33>,
    rdr_gate: RenderGate<'a, GL33>,
}

#[allow(dead_code)]
impl RenderContext<'_> {
    pub(crate) fn new<'a>(pgr_interface: ProgramInterface<'a, GL33>, rdr_gate: RenderGate<'a, GL33>) -> RenderContext<'a> {
        RenderContext {
            pgr_interface,
            rdr_gate
        }
    }

    /// Sets parameter of shader interface
    pub fn set_uniform<T>(&mut self, parameter: &Uniform<T>, value: T)
        where
            T: Uniformable<GL33>
    {
        self.pgr_interface.set(parameter, value);
    }

    /// Prepares render state and runs the closure
    pub fn render<F>(&mut self, state: RenderState, func: F) -> Result<(), PipelineError>
        where
            F: FnOnce(TessContext) -> Result<(), PipelineError>
    {
        self.rdr_gate.render(&state, |tess_gate| {
            func(TessContext::new(tess_gate))
        })
    }
}