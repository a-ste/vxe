use luminance_gl::GL33;
use luminance::tess_gate::TessGate;
use luminance::tess::{TessView, TessIndex};
use luminance::pipeline::PipelineError;
use crate::data::LumTess;
use luminance::vertex::Vertex;

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
    pub fn draw<V, I>(&mut self, tess: &LumTess<V, I>) -> Result<(), PipelineError>
        where
            V: Vertex,
            I: TessIndex,
    {
        self.tess_gate.render(TessView::whole(tess))
    }
}