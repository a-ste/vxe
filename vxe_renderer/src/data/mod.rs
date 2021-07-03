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
pub use luminance_gl::GL33;
use luminance::shader::{Program};
use luminance::tess::{Tess, Interleaved};
use luminance::framebuffer::Framebuffer;
use luminance::texture::Dim2;

/// Backend type for Shader program
pub type LumProgram<I> = Program<GL33, VertexSemantics, (), I>;

/// Backend type for Tesselation, a set of vertices or pretty much all the mesh data that will be sent to GPU
pub type LumTess = Tess<GL33, Vertex, u32, (), Interleaved>;

/// Backend type for Frame Buffers
pub type LumFrameBuffer = Framebuffer<GL33, Dim2, (), ()>;

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
    #[sem(name = "color", repr = "[f32; 3]", wrapper = "VertexRGB")]
    Color,
}

#[derive(Copy, Clone, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    #[allow(dead_code)]
    position: VertexPosition,

    #[allow(dead_code)]
    color: VertexRGB,
}