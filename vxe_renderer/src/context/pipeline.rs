use luminance_gl::GL33;
use luminance::pipeline::{Pipeline};
use luminance::texture::{Texture, Dim2};
use crate::data::shader::luminance::pipeline::{BoundTexture};
use luminance::pixel::Pixel;

/// Pipeline context for binding shaders
#[allow(dead_code)]
pub struct PipelineContext<'a> {
    pipeline: Pipeline<'a, GL33>
}

#[allow(dead_code)]
impl PipelineContext<'_> {
    pub(crate) fn new(pipeline: Pipeline<GL33>) -> PipelineContext {
        PipelineContext {
            pipeline
        }
    }

    /// Binds specified texture and returns binding
    pub fn bind_texture<'a, P>(&'a self, tex: &'a mut Texture<GL33, Dim2, P>) -> BoundTexture<'a, GL33, Dim2, P>
        where
            P: Pixel
    {
        self.pipeline.bind_texture(tex).unwrap()
    }
}