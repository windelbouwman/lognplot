use std::sync::Arc;
/// Function to debug vulkan.
///
use vulkano::instance::debug::{DebugCallback, MessageTypes};
use vulkano::instance::Instance;

/// Install debug message handler:
pub fn enable_logging(instance: &Arc<Instance>) {
    let all = MessageTypes {
        error: true,
        warning: true,
        performance_warning: true,
        information: true,
        debug: true,
    };

    let _debug_callback = DebugCallback::new(instance, all, |msg| {
        use log::Level;
        let level = if msg.ty.error {
            Level::Error
        } else if msg.ty.warning {
            Level::Warn
        } else if msg.ty.performance_warning {
            Level::Warn
        } else if msg.ty.information {
            Level::Info
        } else if msg.ty.debug {
            Level::Debug
        } else {
            panic!("no-impl");
        };

        log!(level, "{}: {}", msg.layer_prefix, msg.description);
    })
    .ok();
}
