mod macros;

use luminance_derive::{Semantics, Vertex};
pub use luminance::pipeline::Viewport;
pub use luminance::scissor::ScissorRegion;
pub use luminance::blending::Blending;
pub use luminance::blending::BlendingMode;
pub use luminance::depth_test::DepthComparison;
pub use luminance::depth_test::DepthWrite;
pub use luminance::face_culling::FaceCulling;
pub use luminance::face_culling::FaceCullingMode;
pub use luminance::face_culling::FaceCullingOrder;
pub use luminance::shader::UniformInterface;
pub use luminance::texture::Sampler;
pub use luminance_gl::GL33;
use luminance::shader::{Program};
use luminance::tess::{Tess, Interleaved};
use luminance::framebuffer::Framebuffer;
use luminance::texture::Dim2;
use luminance::pixel::{NormRGB8UI, NormRGBA8UI, NormR8UI, NormUnsigned, Depth32F, Floating};
use luminance::pipeline::TextureBinding;

/// Backend type for Shader program
pub type LumProgram<I> = Program<GL33, VertexSemantics, (), I>;

/// Backend type for Tesselation, a set of vertices or pretty much all the mesh data that will be sent to GPU
pub type LumTess<V, I> = Tess<GL33, V, I, (), Interleaved>;

/// Backend type for Frame Buffers. C generic must be a tuple of LumRGB, LumRGBA, LumGray; D either empty tuple or LumDepth
pub type LumFrameBuffer<C, D> = Framebuffer<GL33, Dim2, C, D>;

/// Backend type for RGB Color slot
pub type LumRGB = NormRGB8UI;

/// Backend type for RGBA slot
pub type LumRGBA = NormRGBA8UI;

/// Backend type for Gray slot
pub type LumGray = NormR8UI;

/// Backend type for Depth slot
pub type LumDepth = Depth32F;

/// Backend type for a color, required for defining samplers in shader interfaces
pub type LumTextureBinding = TextureBinding<Dim2, NormUnsigned>;

/// Backend type for a color, required for defining samplers in shader interfaces
pub type LumDepthBinding = TextureBinding<Dim2, Floating>;

/// Contains required re-exports for shd_interface macro to work
pub mod shader {
    pub use luminance;
    pub use luminance::UniformInterface;
    pub use luminance::shader::Uniform;
}


#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "normal", repr = "[f32; 3]", wrapper = "VertexNormal")]
    Normal,
    #[sem(name = "color", repr = "[f32; 3]", wrapper = "VertexRGB")]
    Color,
}

#[derive(Copy, Clone, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    #[allow(dead_code)]
    position: VertexPosition,

    #[allow(dead_code)]
    normal: VertexNormal,

    #[allow(dead_code)]
    color: VertexRGB,
}