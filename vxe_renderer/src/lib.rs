//! Renderer crate for vxe_engine
//!
//! At core is a simplification of luminance API, but also provides easy way to render meshes using PBR materials and deferred rendering
//!
//! This crate shouldn't be used by itself, unless you're interested in working on your own engine with help of this renderer crate
#[warn(missing_docs)]

mod renderer;

/// Manages most of data related things around here (Vertices, etc)
pub mod data;

/// Contains contexts that are used for helping with various aspects of rendering things
pub mod context;

/// Contains handler that is used for having control over all events that can happen with a window
pub mod handler;

/// Contains types for handling various aspects of keeping data and using it for rendering (Meshes, Materials, Shaders)
pub mod types;

/// Contains utils and types for easy deferred rendering and PBR
pub mod deferred;

pub use renderer::Renderer;
pub use renderer::builder::RendererBuilder;