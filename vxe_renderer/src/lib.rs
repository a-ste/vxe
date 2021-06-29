//! Renderer crate for vxe_engine
//!
//! Essentially is a simplification of luminance API, a glorious wrapper, so I can have everything work how I want it to work
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



pub use renderer::Renderer;
pub use renderer::builder::RendererBuilder;