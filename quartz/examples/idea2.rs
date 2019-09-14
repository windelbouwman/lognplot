// try out vulkan and cassowary

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;

fn main() {
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");
}
