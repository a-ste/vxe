use super::Renderer;

pub struct RendererBuilder {
    title: String,
    resolution: [u32; 2],
    vsync: bool,
}

impl RendererBuilder {
    pub fn new() -> RendererBuilder {
        RendererBuilder {
            title: "A good game".to_string(),
            resolution: [640, 480],
            vsync: true,
        }
    }

    pub fn title(mut self, title: &str) -> RendererBuilder {
        self.title = title.to_string(); self
    }

    pub fn resolution(mut self, res: [u32; 2]) -> RendererBuilder {
        self.resolution = res; self
    }

    pub fn vsync(mut self, vsync: bool) -> RendererBuilder {
        self.vsync = vsync; self
    }

    pub fn build(self) -> Renderer {
        Renderer::init(self.title, self.resolution, self.vsync)
    }
}