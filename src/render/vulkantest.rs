use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::device::QueueFlags;
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo};
use winit::{
    application::ApplicationHandler, event::*, event_loop::{ActiveEventLoop, EventLoop}, keyboard::{KeyCode, PhysicalKey}, window::Window
};

pub fn vk_init() {
    let library = VulkanLibrary::new().expect("no vulkan library found!");
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            ..Default::default()
        },
    ).expect("failed to create instance");

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");

    for family in physical_device.queue_family_properties() {
        println!("queue family with {:?} queues found", family.queue_count);
    }

    let queue_family_index = physical_device
        .queue_family_properties()
        .iter()
        .position(|queue_family_properties| {
            queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
        })
        .expect("couldn't find a graphical queue family") as u32;

    let (device, mut _queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    let physical = device.physical_device();
    let properties = physical.properties();

    println!(
        "Using device: {} (type: {:?})",
        properties.device_name,
        properties.device_type
    );

    println!(
        "API version: {}.{}.{}",
        properties.api_version.major,
        properties.api_version.minor,
        properties.api_version.patch
    );

}