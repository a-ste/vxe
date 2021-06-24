use super::Instance;

pub struct InstanceBuilder {
    title: String,
    resolution: [u32; 2],
    vsync: bool,
}

impl InstanceBuilder {
    pub fn new() -> InstanceBuilder {
        InstanceBuilder {
            title: "A good game".to_string(),
            resolution: [640, 480],
            vsync: true,
        }
    }

    pub fn title(mut self, title: &str) -> InstanceBuilder {
        self.title = title.to_string(); self
    }

    pub fn resolution(mut self, res: [u32; 2]) -> InstanceBuilder {
        self.resolution = res; self
    }

    pub fn vsync(mut self, vsync: bool) -> InstanceBuilder {
        self.vsync = vsync; self
    }

    pub fn build(self) -> Instance {
        Instance::init(self.title, self.resolution, self.vsync)
    }
}