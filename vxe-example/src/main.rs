use vxe::{InstanceBuilder};

fn main() {
    let mut instance = InstanceBuilder::new()
        .title("hi")
        .build();

    instance.run_loop();
}
