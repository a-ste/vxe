use crate::data::LumFrameBuffer;
use crate::context::{Context, PipelineState};
use crate::data::shader::luminance::pipeline::{Render, PipelineError};
use luminance::pixel::{DepthPixel, ColorPixel, RenderablePixel};

/// Util struct to help managing framebuffers
#[allow(dead_code)]
pub struct FrameUtils {}

#[allow(dead_code)]
impl FrameUtils {
    /// Clears provided frame to black
    pub fn clear_black<C, D>(ctx: &mut Context, frame: &LumFrameBuffer<C, D>) -> Render<PipelineError>
        where
            C: ColorPixel + RenderablePixel,
            D: DepthPixel
    {
        ctx.pipeline(frame, PipelineState::default(), |_, _| Ok(()))
    }

    /// Clears provided frame with specified color
    pub fn clear_frame<C, D>(ctx: &mut Context, frame: &LumFrameBuffer<C, D>, color: [f32; 4]) -> Render<PipelineError>
        where
            C: ColorPixel + RenderablePixel,
            D: DepthPixel
    {
        ctx.pipeline(frame, PipelineState::default().set_clear_color(color), |_, _| Ok(()))
    }
}