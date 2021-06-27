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

#[derive(Copy, Clone, Debug, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}

#[derive(Copy, Clone, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    #[allow(dead_code)]
    position: VertexPosition,

    #[allow(dead_code)]
    #[vertex(normalized = "true")]
    color: VertexRGB,
}