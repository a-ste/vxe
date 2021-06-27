use super::Renderer;

/// Renderer builder, so instancing it would make more sense
pub struct RendererBuilder {
    title: String,
    resolution: [u32; 2],
    vsync: bool,
}

impl RendererBuilder {
    /// Creates builder for Renderer
    pub fn new() -> RendererBuilder {
        RendererBuilder {
            title: "A good game".to_string(),
            resolution: [640, 480],
            vsync: true,
        }
    }

    /// Sets title of the window
    pub fn title(mut self, title: &str) -> RendererBuilder {
        self.title = title.to_string(); self
    }

    /// Sets resolution for the window
    pub fn resolution(mut self, res: [u32; 2]) -> RendererBuilder {
        self.resolution = res; self
    }

    /// Sets vsync mode
    pub fn vsync(mut self, vsync: bool) -> RendererBuilder {
        self.vsync = vsync; self
    }

    /// Builds the Renderer
    pub fn build(self) -> Renderer {
        Renderer::init(self.title, self.resolution, self.vsync)
    }
}