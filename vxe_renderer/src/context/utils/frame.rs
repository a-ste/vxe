use crate::data::{LumFrameBuffer, GL33};
use crate::context::{Context, PipelineState};
use crate::data::shader::luminance::pipeline::{Render, PipelineError};
use luminance::backend::color_slot::ColorSlot;
use luminance::texture::Dim2;
use luminance::backend::depth_slot::DepthSlot;

/// Util struct to help managing framebuffers
#[allow(dead_code)]
pub struct FrameUtils {}

#[allow(dead_code)]
impl FrameUtils {
    /// Clears provided frame to black
    pub fn clear_black<C, D>(ctx: &mut Context, frame: &LumFrameBuffer<C, D>) -> Render<PipelineError>
        where
            C: ColorSlot<GL33, Dim2>,
            D: DepthSlot<GL33, Dim2>
    {
        ctx.pipeline(frame, PipelineState::default(), |_, _| Ok(()))
    }

    /// Clears provided frame with specified color
    pub fn clear_frame<C, D>(ctx: &mut Context, frame: &LumFrameBuffer<C, D>, color: [f32; 4]) -> Render<PipelineError>
        where
            C: ColorSlot<GL33, Dim2>,
            D: DepthSlot<GL33, Dim2>
    {
        ctx.pipeline(frame, PipelineState::default().set_clear_color(color), |_, _| Ok(()))
    }
}