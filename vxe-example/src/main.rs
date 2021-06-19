use vxe::{InstanceBuilder};

fn main() {
    let mut instance = InstanceBuilder::new()
        .title("hi")
        .vsync(true)
        .build();

    instance.run_loop();
}
