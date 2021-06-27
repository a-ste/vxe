use luminance_gl::GL33;
use luminance::tess_gate::TessGate;
use crate::context::LumTess;
use luminance::tess::TessView;
use luminance::pipeline::PipelineError;

/// Tesselation context for drawing triangles
#[allow(dead_code)]
pub struct TessContext<'a> {
    tess_gate: TessGate<'a, GL33>
}

#[allow(dead_code)]
impl TessContext<'_> {
    pub(crate) fn new(tess_gate: TessGate<GL33>) -> TessContext {
        TessContext {
            tess_gate
        }
    }

    /// Draws tesselation
    pub fn draw(&mut self, tess: &LumTess) -> Result<(), PipelineError> {
        self.tess_gate.render(TessView::whole(tess))
    }
}