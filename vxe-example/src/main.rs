use vxe_renderer::{RendererBuilder};

fn main() {
    let mut renderer = RendererBuilder::new()
        .title("hi")
        .vsync(true)
        .build();

    renderer.run_loop();
}
