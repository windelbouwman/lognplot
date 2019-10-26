use super::vulkan::VulkanEngine;

pub trait Paintable {
    fn paint(&self, engine: &mut VulkanEngine);
}
