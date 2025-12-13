use ash::vk;
use ash::{Device, Entry, Instance};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::ffi::CString;
use std::sync::Arc;
use winit::{event_loop::EventLoop, window::Window};

use crate::*;
use crate::shared::*;
use crate::shared::asset::*;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VkVertex {
    position: [f32; 2],
    color: [f32; 3],
}

#[allow(dead_code)]
pub struct VkState {
    pub entry: Entry,
    pub instance: Instance,
    pub device: Device,
    pub physical_device: vk::PhysicalDevice,
    pub queue: vk::Queue,
    pub queue_family_index: u32,
    pub surface_loader: ash::khr::surface::Instance,
    pub surface: vk::SurfaceKHR,
    pub swapchain_loader: ash::khr::swapchain::Device,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
    pub render_pass: vk::RenderPass,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub vertex_buffer: vk::Buffer,
    pub vertex_buffer_memory: vk::DeviceMemory,
    pub image_available_semaphores: Vec<vk::Semaphore>,
    pub render_finished_semaphores: Vec<vk::Semaphore>,
    pub in_flight_fences: Vec<vk::Fence>,
    pub current_frame: usize,
    pub recreate_swapchain: bool,
    pub vs_module: vk::ShaderModule,
    pub fs_module: vk::ShaderModule,
}

const MAX_FRAMES_IN_FLIGHT: usize = 2;

thread_local! {
    static VK_STATE: std::cell::RefCell<Option<VkState>> = std::cell::RefCell::new(None);
}

unsafe fn create_instance(entry: &Entry, window: &Window) -> Instance {
    let app_name = CString::new("Nova GE").unwrap();
    let engine_name = CString::new("Nova Engine").unwrap();
    
    let app_info = vk::ApplicationInfo {
        p_application_name: app_name.as_ptr(),
        application_version: vk::make_api_version(0, 1, 0, 0),
        p_engine_name: engine_name.as_ptr(),
        engine_version: vk::make_api_version(0, 1, 0, 0),
        api_version: vk::API_VERSION_1_2,
        ..Default::default()
    };

    let extension_names = ash_window::enumerate_required_extensions(
        window.display_handle().unwrap().as_raw()
    ).unwrap().to_vec();

    let create_info = vk::InstanceCreateInfo {
        p_application_info: &app_info,
        enabled_extension_count: extension_names.len() as u32,
        pp_enabled_extension_names: extension_names.as_ptr(),
        ..Default::default()
    };

    unsafe {
        entry.create_instance(&create_info, None).expect("Failed to create instance")
    }
}

unsafe fn pick_physical_device(
    instance: &Instance,
    surface_loader: &ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
) -> (vk::PhysicalDevice, u32) {
    unsafe {
        let devices = instance.enumerate_physical_devices().expect("Failed to enumerate physical devices");
    
        for device in devices {
            let queue_families = instance.get_physical_device_queue_family_properties(device);
        
            for (i, queue_family) in queue_families.iter().enumerate() {
                let supports_graphics = queue_family.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                let supports_surface = surface_loader
                    .get_physical_device_surface_support(device, i as u32, surface)
                    .unwrap_or(false);
            
                if supports_graphics && supports_surface {
                    return (device, i as u32);
                }
            }
        }
    
        panic!("Failed to find suitable physical device");
    }
}

unsafe fn create_logical_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_family_index: u32,
) -> (Device, vk::Queue) {
    let queue_priorities = [1.0];
    let queue_create_info = vk::DeviceQueueCreateInfo {
        queue_family_index,
        queue_count: 1,
        p_queue_priorities: queue_priorities.as_ptr(),
        ..Default::default()
    };

    let device_extension_names = [ash::khr::swapchain::NAME.as_ptr()];

    let device_create_info = vk::DeviceCreateInfo {
        queue_create_info_count: 1,
        p_queue_create_infos: &queue_create_info,
        enabled_extension_count: device_extension_names.len() as u32,
        pp_enabled_extension_names: device_extension_names.as_ptr(),
        ..Default::default()
    };

    unsafe {
        let device = instance
            .create_device(physical_device, &device_create_info, None)
            .expect("Failed to create logical device");

        let queue = device.get_device_queue(queue_family_index, 0);

        (device, queue)
    }
}

unsafe fn create_swapchain(
    instance: &Instance,
    device: &Device,
    physical_device: vk::PhysicalDevice,
    surface_loader: &ash::khr::surface::Instance,
    surface: vk::SurfaceKHR,
    window: &Window,
) -> (ash::khr::swapchain::Device, vk::SwapchainKHR, Vec<vk::Image>, vk::Format, vk::Extent2D) {
    unsafe {
        let capabilities = surface_loader
            .get_physical_device_surface_capabilities(physical_device, surface)
            .unwrap();

        let formats = surface_loader
            .get_physical_device_surface_formats(physical_device, surface)
            .unwrap();

        let format = formats
            .iter()
            .find(|f| f.format == vk::Format::B8G8R8A8_UNORM || f.format == vk::Format::R8G8B8A8_UNORM)
            .unwrap_or(&formats[0]);

        let present_modes = surface_loader
            .get_physical_device_surface_present_modes(physical_device, surface)
            .unwrap();
    

        let present_mode = if present_modes.contains(&vk::PresentModeKHR::MAILBOX) {
            vk::PresentModeKHR::MAILBOX
        } else {
            vk::PresentModeKHR::FIFO
        };

        let window_size = window.inner_size();
        let extent = vk::Extent2D {
            width: window_size.width.clamp(capabilities.min_image_extent.width, capabilities.max_image_extent.width),
            height: window_size.height.clamp(capabilities.min_image_extent.height, capabilities.max_image_extent.height),
        };

        let image_count = (capabilities.min_image_count + 1).min(
            if capabilities.max_image_count == 0 {
                u32::MAX
            } else {
                capabilities.max_image_count
            },
        );

        let swapchain_create_info = vk::SwapchainCreateInfoKHR {
            surface,
            min_image_count: image_count,
            image_format: format.format,
            image_color_space: format.color_space,
            image_extent: extent,
            image_array_layers: 1,
            image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
            image_sharing_mode: vk::SharingMode::EXCLUSIVE,
            pre_transform: capabilities.current_transform,
            composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
            present_mode,
            clipped: vk::TRUE,
            ..Default::default()
        };

        let swapchain_loader = ash::khr::swapchain::Device::new(instance, device);
        let swapchain = swapchain_loader
            .create_swapchain(&swapchain_create_info, None)
            .expect("Failed to create swapchain");

        let images = swapchain_loader.get_swapchain_images(swapchain).unwrap();

        (swapchain_loader, swapchain, images, format.format, extent)
    }
}

unsafe fn create_image_views(
    device: &Device,
    images: &[vk::Image],
    format: vk::Format,
) -> Vec<vk::ImageView> {
    images
        .iter()
        .map(|&image| {
            let create_info = vk::ImageViewCreateInfo {
                image,
                view_type: vk::ImageViewType::TYPE_2D,
                format,
                components: vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                },
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                },
                ..Default::default()
            };
            unsafe {
                device.create_image_view(&create_info, None).unwrap()
            }
        })
        .collect()
}

unsafe fn create_render_pass(device: &Device, format: vk::Format) -> vk::RenderPass {
    let color_attachment = vk::AttachmentDescription {
        format,
        samples: vk::SampleCountFlags::TYPE_1,
        load_op: vk::AttachmentLoadOp::CLEAR,
        store_op: vk::AttachmentStoreOp::STORE,
        stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
        stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
        initial_layout: vk::ImageLayout::UNDEFINED,
        final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        ..Default::default()
    };

    let color_attachment_ref = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
    };

    let subpass = vk::SubpassDescription {
        pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
        color_attachment_count: 1,
        p_color_attachments: &color_attachment_ref,
        ..Default::default()
    };

    let dependency = vk::SubpassDependency {
        src_subpass: vk::SUBPASS_EXTERNAL,
        dst_subpass: 0,
        src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        src_access_mask: vk::AccessFlags::empty(),
        dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
        dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
        ..Default::default()
    };

    let render_pass_info = vk::RenderPassCreateInfo {
        attachment_count: 1,
        p_attachments: &color_attachment,
        subpass_count: 1,
        p_subpasses: &subpass,
        dependency_count: 1,
        p_dependencies: &dependency,
        ..Default::default()
    };

    unsafe {
        device.create_render_pass(&render_pass_info, None).unwrap()
    }
}

unsafe fn create_shader_module(device: &Device, code: &[u32]) -> vk::ShaderModule {
    let create_info = vk::ShaderModuleCreateInfo {
        code_size: code.len() * std::mem::size_of::<u32>(),
        p_code: code.as_ptr(),
        ..Default::default()
    };

    unsafe {
        device.create_shader_module(&create_info, None).unwrap()
    }
}

unsafe fn create_graphics_pipeline(
    device: &Device,
    extent: vk::Extent2D,
    render_pass: vk::RenderPass,
    vs_module: vk::ShaderModule,
    fs_module: vk::ShaderModule,
) -> (vk::Pipeline, vk::PipelineLayout) {
    let entry_point = CString::new("main").unwrap();

    let vs_stage_info = vk::PipelineShaderStageCreateInfo {
        stage: vk::ShaderStageFlags::VERTEX,
        module: vs_module,
        p_name: entry_point.as_ptr(),
        ..Default::default()
    };

    let fs_stage_info = vk::PipelineShaderStageCreateInfo {
        stage: vk::ShaderStageFlags::FRAGMENT,
        module: fs_module,
        p_name: entry_point.as_ptr(),
        ..Default::default()
    };

    let shader_stages = [vs_stage_info, fs_stage_info];

    let binding_description = vk::VertexInputBindingDescription {
        binding: 0,
        stride: std::mem::size_of::<VkVertex>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    };

    let attribute_descriptions = [
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: 0,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 1,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: 8,
        },
    ];

    let vertex_input_info = vk::PipelineVertexInputStateCreateInfo {
        vertex_binding_description_count: 1,
        p_vertex_binding_descriptions: &binding_description,
        vertex_attribute_description_count: attribute_descriptions.len() as u32,
        p_vertex_attribute_descriptions: attribute_descriptions.as_ptr(),
        ..Default::default()
    };

    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        primitive_restart_enable: vk::FALSE,
        ..Default::default()
    };

    let viewport = vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: extent.width as f32,
        height: extent.height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    };

    let scissor = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent,
    };

    let viewport_state = vk::PipelineViewportStateCreateInfo {
        viewport_count: 1,
        p_viewports: &viewport,
        scissor_count: 1,
        p_scissors: &scissor,
        ..Default::default()
    };

    let rasterizer = vk::PipelineRasterizationStateCreateInfo {
        depth_clamp_enable: vk::FALSE,
        rasterizer_discard_enable: vk::FALSE,
        polygon_mode: vk::PolygonMode::FILL,
        line_width: 1.0,
        cull_mode: vk::CullModeFlags::BACK,
        front_face: vk::FrontFace::CLOCKWISE,
        depth_bias_enable: vk::FALSE,
        ..Default::default()
    };

    let multisampling = vk::PipelineMultisampleStateCreateInfo {
        sample_shading_enable: vk::FALSE,
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        ..Default::default()
    };

    let color_blend_attachment = vk::PipelineColorBlendAttachmentState {
        color_write_mask: vk::ColorComponentFlags::RGBA,
        blend_enable: vk::FALSE,
        ..Default::default()
    };

    let color_blending = vk::PipelineColorBlendStateCreateInfo {
        logic_op_enable: vk::FALSE,
        attachment_count: 1,
        p_attachments: &color_blend_attachment,
        ..Default::default()
    };

    unsafe {
        let pipeline_layout_info = vk::PipelineLayoutCreateInfo::default();
        let pipeline_layout = device
            .create_pipeline_layout(&pipeline_layout_info, None)
            .unwrap();

        let pipeline_info = vk::GraphicsPipelineCreateInfo {
            stage_count: shader_stages.len() as u32,
            p_stages: shader_stages.as_ptr(),
            p_vertex_input_state: &vertex_input_info,
            p_input_assembly_state: &input_assembly,
            p_viewport_state: &viewport_state,
            p_rasterization_state: &rasterizer,
            p_multisample_state: &multisampling,
            p_color_blend_state: &color_blending,
            layout: pipeline_layout,
            render_pass,
            subpass: 0,
            ..Default::default()
        };

        let pipeline = device
            .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_info], None)
            .expect("Failed to create graphics pipeline")[0];

        (pipeline, pipeline_layout)
    }
}

unsafe fn create_framebuffers(
    device: &Device,
    image_views: &[vk::ImageView],
    render_pass: vk::RenderPass,
    extent: vk::Extent2D,
) -> Vec<vk::Framebuffer> {
    image_views
        .iter()
        .map(|&image_view| {
            let attachments = [image_view];
            let framebuffer_info = vk::FramebufferCreateInfo {
                render_pass,
                attachment_count: 1,
                p_attachments: attachments.as_ptr(),
                width: extent.width,
                height: extent.height,
                layers: 1,
                ..Default::default()
            };

            unsafe {
                device.create_framebuffer(&framebuffer_info, None).unwrap()
            }
        })
        .collect()
}

unsafe fn find_memory_type(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    type_filter: u32,
    properties: vk::MemoryPropertyFlags,
) -> u32 {
    unsafe {
        let mem_properties = instance.get_physical_device_memory_properties(physical_device);

        for i in 0..mem_properties.memory_type_count {
            if (type_filter & (1 << i)) != 0
                && mem_properties.memory_types[i as usize]
                    .property_flags
                    .contains(properties)
            {
                return i;
            }
        }

        panic!("Failed to find suitable memory type");
    }
}

unsafe fn create_vertex_buffer(
    instance: &Instance,
    device: &Device,
    physical_device: vk::PhysicalDevice,
) -> (vk::Buffer, vk::DeviceMemory) {
    let vertices = [
        VkVertex {
            position: [0.0, -0.5],
            color: [1.0, 0.0, 0.0],
        },
        VkVertex {
            position: [0.5, 0.5],
            color: [0.0, 1.0, 0.0],
        },
        VkVertex {
            position: [-0.5, 0.5],
            color: [0.0, 0.0, 1.0],
        },
    ];

    let buffer_size = std::mem::size_of_val(&vertices) as vk::DeviceSize;

    let buffer_info = vk::BufferCreateInfo {
        size: buffer_size,
        usage: vk::BufferUsageFlags::VERTEX_BUFFER,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        ..Default::default()
    };

    unsafe {
        let buffer = device.create_buffer(&buffer_info, None).unwrap();
        let mem_requirements = device.get_buffer_memory_requirements(buffer);

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: mem_requirements.size,
            memory_type_index: find_memory_type(
                instance,
                physical_device,
                mem_requirements.memory_type_bits,
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
            ),
            ..Default::default()
        };

        let buffer_memory = device.allocate_memory(&alloc_info, None).unwrap();
        device.bind_buffer_memory(buffer, buffer_memory, 0).unwrap();

        let data_ptr = device
            .map_memory(buffer_memory, 0, buffer_size, vk::MemoryMapFlags::empty())
            .unwrap();
        std::ptr::copy_nonoverlapping(vertices.as_ptr(), data_ptr as *mut VkVertex, vertices.len());
        device.unmap_memory(buffer_memory);

        (buffer, buffer_memory)
    }
}

unsafe fn create_command_pool(device: &Device, queue_family_index: u32) -> vk::CommandPool {
    let pool_info = vk::CommandPoolCreateInfo {
        flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
        queue_family_index,
        ..Default::default()
    };

    unsafe {
        device.create_command_pool(&pool_info, None).unwrap()
    }
}

unsafe fn create_command_buffers(
    device: &Device,
    command_pool: vk::CommandPool,
    count: usize,
) -> Vec<vk::CommandBuffer> {
    let alloc_info = vk::CommandBufferAllocateInfo {
        command_pool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: count as u32,
        ..Default::default()
    };

    unsafe {
        device.allocate_command_buffers(&alloc_info).unwrap()
    }
}

unsafe fn create_sync_objects(device: &Device) -> (Vec<vk::Semaphore>, Vec<vk::Semaphore>, Vec<vk::Fence>) {
    let mut image_available_semaphores = Vec::new();
    let mut render_finished_semaphores = Vec::new();
    let mut in_flight_fences = Vec::new();

    let semaphore_info = vk::SemaphoreCreateInfo::default();
    let fence_info = vk::FenceCreateInfo {
        flags: vk::FenceCreateFlags::SIGNALED,
        ..Default::default()
    };

    unsafe {
        for _ in 0..MAX_FRAMES_IN_FLIGHT {
            image_available_semaphores.push(device.create_semaphore(&semaphore_info, None).unwrap());
            render_finished_semaphores.push(device.create_semaphore(&semaphore_info, None).unwrap());
            in_flight_fences.push(device.create_fence(&fence_info, None).unwrap());
        }

        (image_available_semaphores, render_finished_semaphores, in_flight_fences)
    }
}

pub fn vk_init(shared: &SharedData, window: &Arc<Window>, _event_loop: &EventLoop<()>) {
    unsafe {
        let entry = Entry::load().expect("Failed to load Vulkan");
        let instance = create_instance(&entry, window);
        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
        let surface = ash_window::create_surface(
            &entry,
            &instance,
            window.display_handle().unwrap().as_raw(),
            window.window_handle().unwrap().as_raw(),
            None
        ).unwrap();

        let (physical_device, queue_family_index) = pick_physical_device(&instance, &surface_loader, surface);
        let (device, queue) = create_logical_device(&instance, physical_device, queue_family_index);

        let (swapchain_loader, swapchain, swapchain_images, swapchain_format, swapchain_extent) =
            create_swapchain(&instance, &device, physical_device, &surface_loader, surface, window);

        let swapchain_image_views = create_image_views(&device, &swapchain_images, swapchain_format);
        let render_pass = create_render_pass(&device, swapchain_format);

        let mut vs_data = Vec::<u32>::new();
        let mut fs_data = Vec::<u32>::new();

        let vs_ass = get_asset(shared, "test_vs");
        let fs_ass = get_asset(shared, "test_fs");

        if let Asset::Shader(mut data) = vs_ass {
            vs_data.append(&mut data);
        } else {
            err!(shared, "Failed to get the test vertex shader");
        }

        if let Asset::Shader(mut data) = fs_ass {
            fs_data.append(&mut data);
        } else {
            err!(shared, "Failed to get the test fragment shader");
        }

        let vs_module = create_shader_module(&device, &vs_data);
        let fs_module = create_shader_module(&device, &fs_data);

        let (pipeline, pipeline_layout) = create_graphics_pipeline(&device, swapchain_extent, render_pass, vs_module, fs_module);

        let framebuffers = create_framebuffers(&device, &swapchain_image_views, render_pass, swapchain_extent);
        let command_pool = create_command_pool(&device, queue_family_index);
        
        let (vertex_buffer, vertex_buffer_memory) = create_vertex_buffer(&instance, &device, physical_device);

        let command_buffers = create_command_buffers(&device, command_pool, MAX_FRAMES_IN_FLIGHT);
        let (image_available_semaphores, render_finished_semaphores, in_flight_fences) = create_sync_objects(&device);

        let state = VkState {
            entry,
            instance,
            device,
            physical_device,
            queue,
            queue_family_index,
            surface_loader,
            surface,
            swapchain_loader,
            swapchain,
            swapchain_images,
            swapchain_image_views,
            swapchain_format,
            swapchain_extent,
            render_pass,
            pipeline_layout,
            pipeline,
            framebuffers,
            command_pool,
            command_buffers,
            vertex_buffer,
            vertex_buffer_memory,
            image_available_semaphores,
            render_finished_semaphores,
            in_flight_fences,
            current_frame: 0,
            recreate_swapchain: false,
            vs_module,
            fs_module,
        };

        VK_STATE.with(|vk| {
            *vk.borrow_mut() = Some(state);
        });
    }
}

pub fn vk_reload_shaders(vert_spirv: &[u32], frag_spirv: &[u32]) {
    VK_STATE.with(|vk| {
        let mut state_opt = vk.borrow_mut();
        let state = match state_opt.as_mut() {
            Some(s) => s,
            None => return,
        };

        unsafe {
            state.device.device_wait_idle().unwrap();

            state.device.destroy_pipeline(state.pipeline, None);
            state.device.destroy_pipeline_layout(state.pipeline_layout, None);
            state.device.destroy_shader_module(state.vs_module, None);
            state.device.destroy_shader_module(state.fs_module, None);

            state.vs_module = create_shader_module(&state.device, vert_spirv);
            state.fs_module = create_shader_module(&state.device, frag_spirv);

            let (pipeline, pipeline_layout) = create_graphics_pipeline(
                &state.device,
                state.swapchain_extent,
                state.render_pass,
                state.vs_module,
                state.fs_module,
            );

            state.pipeline = pipeline;
            state.pipeline_layout = pipeline_layout;
        }
    });
}

unsafe fn recreate_swapchain_impl(state: &mut VkState, window: &Window) {
    unsafe {
        state.device.device_wait_idle().unwrap();

        for &framebuffer in &state.framebuffers {
            state.device.destroy_framebuffer(framebuffer, None);
        }
        for &image_view in &state.swapchain_image_views {
            state.device.destroy_image_view(image_view, None);
        }
        state.swapchain_loader.destroy_swapchain(state.swapchain, None);

        let (swapchain_loader, swapchain, swapchain_images, swapchain_format, swapchain_extent) =
            create_swapchain(&state.instance, &state.device, state.physical_device, &state.surface_loader, state.surface, window);

        state.swapchain_loader = swapchain_loader;
        state.swapchain = swapchain;
        state.swapchain_images = swapchain_images;
        state.swapchain_format = swapchain_format;
        state.swapchain_extent = swapchain_extent;

        state.swapchain_image_views = create_image_views(&state.device, &state.swapchain_images, state.swapchain_format);
        state.framebuffers = create_framebuffers(&state.device, &state.swapchain_image_views, state.render_pass, state.swapchain_extent);

        state.device.destroy_pipeline(state.pipeline, None);
        state.device.destroy_pipeline_layout(state.pipeline_layout, None);

        let (pipeline, pipeline_layout) = create_graphics_pipeline(
            &state.device,
            state.swapchain_extent,
            state.render_pass,
            state.vs_module,
            state.fs_module,
        );

        state.pipeline = pipeline;
        state.pipeline_layout = pipeline_layout;
    }
}

pub fn vk_render(window: &Arc<Window>) {
    VK_STATE.with(|vk| {
        let mut state_opt = vk.borrow_mut();
        let state = match state_opt.as_mut() {
            Some(s) => s,
            None => return,
        };

        let window_size = window.inner_size();
        if window_size.width == 0 || window_size.height == 0 {
            return;
        }

        unsafe {
            if state.recreate_swapchain {
                recreate_swapchain_impl(state, window);
                state.recreate_swapchain = false;
                return;
            }

            let current_frame = state.current_frame;

            state.device
                .wait_for_fences(&[state.in_flight_fences[current_frame]], true, u64::MAX)
                .unwrap();

            let (image_index, suboptimal) = match state.swapchain_loader.acquire_next_image(
                state.swapchain,
                u64::MAX,
                state.image_available_semaphores[current_frame],
                vk::Fence::null(),
            ) {
                Ok(result) => result,
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) => {
                    state.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("Failed to acquire swapchain image: {:?}", e),
            };

            if suboptimal {
                state.recreate_swapchain = true;
            }

            state.device.reset_fences(&[state.in_flight_fences[current_frame]]).unwrap();

            let command_buffer = state.command_buffers[current_frame];
            state.device.reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty()).unwrap();

            let begin_info = vk::CommandBufferBeginInfo::default();
            state.device.begin_command_buffer(command_buffer, &begin_info).unwrap();

            let clear_values = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 0.0, 1.0],
                },
            }];

            let render_pass_begin_info = vk::RenderPassBeginInfo {
                render_pass: state.render_pass,
                framebuffer: state.framebuffers[image_index as usize],
                render_area: vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: state.swapchain_extent,
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
                ..Default::default()
            };

            state.device.cmd_begin_render_pass(
                command_buffer,
                &render_pass_begin_info,
                vk::SubpassContents::INLINE,
            );

            state.device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                state.pipeline,
            );

            state.device.cmd_bind_vertex_buffers(
                command_buffer,
                0,
                &[state.vertex_buffer],
                &[0],
            );

            state.device.cmd_draw(command_buffer, 3, 1, 0, 0);

            state.device.cmd_end_render_pass(command_buffer);

            state.device.end_command_buffer(command_buffer).unwrap();

            let wait_semaphores = [state.image_available_semaphores[current_frame]];
            let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
            let signal_semaphores = [state.render_finished_semaphores[current_frame]];
            let command_buffers = [command_buffer];

            let submit_info = vk::SubmitInfo {
                wait_semaphore_count: 1,
                p_wait_semaphores: wait_semaphores.as_ptr(),
                p_wait_dst_stage_mask: wait_stages.as_ptr(),
                command_buffer_count: 1,
                p_command_buffers: command_buffers.as_ptr(),
                signal_semaphore_count: 1,
                p_signal_semaphores: signal_semaphores.as_ptr(),
                ..Default::default()
            };

            state.device
                .queue_submit(state.queue, &[submit_info], state.in_flight_fences[current_frame])
                .unwrap();

            let swapchains = [state.swapchain];
            let image_indices = [image_index];

            let present_info = vk::PresentInfoKHR {
                wait_semaphore_count: 1,
                p_wait_semaphores: signal_semaphores.as_ptr(),
                swapchain_count: 1,
                p_swapchains: swapchains.as_ptr(),
                p_image_indices: image_indices.as_ptr(),
                ..Default::default()
            };

            match state.swapchain_loader.queue_present(state.queue, &present_info) {
                Ok(_) => {}
                Err(vk::Result::ERROR_OUT_OF_DATE_KHR) | Err(vk::Result::SUBOPTIMAL_KHR) => {
                    state.recreate_swapchain = true;
                }
                Err(e) => panic!("Failed to present: {:?}", e),
            }

            state.current_frame = (state.current_frame + 1) % MAX_FRAMES_IN_FLIGHT;
        }
    });
}

pub fn vk_handle_resize() {
    VK_STATE.with(|vk| {
        if let Some(state) = vk.borrow_mut().as_mut() {
            state.recreate_swapchain = true;
        }
    });
}

impl Drop for VkState {
    fn drop(&mut self) {
        unsafe {
            self.device.device_wait_idle().unwrap();

            for i in 0..MAX_FRAMES_IN_FLIGHT {
                self.device.destroy_semaphore(self.image_available_semaphores[i], None);
                self.device.destroy_semaphore(self.render_finished_semaphores[i], None);
                self.device.destroy_fence(self.in_flight_fences[i], None);
            }

            self.device.destroy_buffer(self.vertex_buffer, None);
            self.device.free_memory(self.vertex_buffer_memory, None);

            self.device.destroy_command_pool(self.command_pool, None);

            for &framebuffer in &self.framebuffers {
                self.device.destroy_framebuffer(framebuffer, None);
            }

            self.device.destroy_pipeline(self.pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);

            self.device.destroy_render_pass(self.render_pass, None);

            for &image_view in &self.swapchain_image_views {
                self.device.destroy_image_view(image_view, None);
            }

            self.swapchain_loader.destroy_swapchain(self.swapchain, None);

            self.device.destroy_shader_module(self.vs_module, None);
            self.device.destroy_shader_module(self.fs_module, None);

            self.device.destroy_device(None);
            self.surface_loader.destroy_surface(self.surface, None);
            self.instance.destroy_instance(None);
        }
    }
}