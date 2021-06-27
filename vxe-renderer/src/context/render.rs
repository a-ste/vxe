use luminance_gl::GL33;
use luminance::render_gate::RenderGate;
use luminance::pipeline::PipelineError;
use crate::context::tess::TessContext;
use luminance::render_state::RenderState;

/// Struct that will be passed whenever you bind a shader, has useful functions for actually rendering things
#[allow(dead_code)]
pub struct RenderContext<'a> {
    rdr_gate: RenderGate<'a, GL33>
}

#[allow(dead_code)]
impl RenderContext<'_> {
    pub(crate) fn new(rdr_gate: RenderGate<GL33>) -> RenderContext {
        RenderContext {
            rdr_gate
        }
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