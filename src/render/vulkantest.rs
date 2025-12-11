use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents};
use vulkano::image::view::ImageView;
use vulkano::pipeline::graphics::color_blend::{ColorBlendAttachmentState, ColorBlendState};
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::multisample::MultisampleState;
use vulkano::pipeline::graphics::rasterization::RasterizationState;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::graphics::vertex_input::{Vertex, VertexDefinition};
use vulkano::pipeline::layout::PipelineDescriptorSetLayoutCreateInfo;
use vulkano::pipeline::{DynamicState, PipelineLayout, PipelineShaderStageCreateInfo};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass};
use vulkano::{VulkanLibrary, Validated, VulkanError};
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::image::{Image, ImageUsage};
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, QueueCreateInfo, QueueFlags, Queue};
use vulkano::memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator};
use vulkano::swapchain::{Surface, Swapchain, SwapchainCreateInfo, acquire_next_image, SwapchainPresentInfo};
use vulkano::buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::graphics::GraphicsPipelineCreateInfo;
use vulkano::sync::{self, GpuFuture};

use std::sync::Arc;
use winit::{event_loop::EventLoop, window::Window};

#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct VkVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
    #[format(R32G32B32_SFLOAT)]
    color: [f32; 3],
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: r"
            #version 460

            layout(location = 0) in vec2 position;
            layout(location = 1) in vec3 color;

            layout(location = 0) out vec3 fragColor;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                fragColor = color;
            }
        ",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: r"
            #version 460

            layout(location = 0) in vec3 fragColor;
            layout(location = 0) out vec4 outColor;

            void main() {
                outColor = vec4(fragColor, 1.0);
            }
        ",
    }
}

pub struct VkState {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<Image>>,
    pub render_pass: Arc<RenderPass>,
    pub pipeline: Arc<GraphicsPipeline>,
    pub viewport: Viewport,
    pub framebuffers: Vec<Arc<Framebuffer>>,
    pub vertex_buffer: Subbuffer<[VkVertex]>,
    pub command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    pub recreate_swapchain: bool,
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,
}

thread_local! {
    static VK_STATE: std::cell::RefCell<Option<VkState>> = std::cell::RefCell::new(None);
}

pub fn vk_init(window: Arc<Window>, event_loop: &EventLoop<()>) {
    let library = VulkanLibrary::new().expect("no vulkan library found!");
    
    let required_extensions = Surface::required_extensions(&event_loop);
    
    let instance = Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    ).expect("failed to create instance");

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::empty()
    };

    let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter(|p| p.supported_extensions().contains(&device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.intersects(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, &surface).unwrap_or(false)
                })
                .map(|i| (p, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        }).unwrap();

    let (device, mut queues) = Device::new(
        physical_device.clone(),
        DeviceCreateInfo {
            enabled_extensions: device_extensions,
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    ).unwrap();

    let queue = queues.next().unwrap();

    let (swapchain, images) = {
        let surface_capabilities = device
            .physical_device()
            .surface_capabilities(&surface, Default::default())
            .unwrap();

        let image_format = device
            .physical_device()
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        Swapchain::new(
            device.clone(),
            surface,
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count.max(2),
                image_format,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .unwrap(),
                ..Default::default()
            },
        ).unwrap()
    };

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

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

    let vertex_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        vertices
    ).unwrap();

    let render_pass = vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                format: swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [color],
            depth_stencil: {}
        },
    ).unwrap();

    let pipeline = {
        let vs = vs::load(device.clone()).expect("failed to create shader module").entry_point("main").unwrap();
        let fs = fs::load(device.clone()).expect("failed to create shader module").entry_point("main").unwrap();
        // Use VertexDefinition to convert to a VertexInputState.
        let vertex_input_state = VkVertex::per_vertex()
            .definition(&vs.info().input_interface)
            .unwrap();
        let viewport_state = ViewportState::default();
        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        ).unwrap();
        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

        GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState::default()),
                viewport_state: Some(viewport_state),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            }
        ).unwrap()
    };

    // Fix viewport extent type to [f32; 2].
    let window_size = window.inner_size();
    let mut viewport = Viewport {
        offset: [0.0, 0.0],
        extent: [window_size.width as f32, window_size.height as f32],
        depth_range: 0.0..=1.0,
    };

    let framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let previous_frame_end = Some(sync::now(device.clone()).boxed());

    let state = VkState {
        device,
        queue,
        swapchain,
        images,
        render_pass,
        pipeline,
        viewport,
        framebuffers,
        vertex_buffer,
        command_buffer_allocator,
        recreate_swapchain: false,
        previous_frame_end,
    };

    VK_STATE.with(|vk| {
        *vk.borrow_mut() = Some(state);
    });
}

// vk_render now takes Arc<Window> instead of &Window.
pub fn vk_render(window: Arc<Window>) {
    VK_STATE.with(|vk| {
        let mut state_opt = vk.borrow_mut();
        let state = match state_opt.as_mut() {
            Some(s) => s,
            None => return,
        };

        let image_extent: [u32; 2] = window.inner_size().into();

        if image_extent.contains(&0) {
            return;
        }

        state.previous_frame_end.as_mut().unwrap().cleanup_finished();

        if state.recreate_swapchain {
            let (new_swapchain, new_images) = state.swapchain
                .recreate(SwapchainCreateInfo {
                    image_extent,
                    ..state.swapchain.create_info()
                })
                .expect("failed to recreate swapchain");

            state.swapchain = new_swapchain;
            state.framebuffers = window_size_dependent_setup(
                &new_images,
                state.render_pass.clone(),
                &mut state.viewport,
            );
            state.images = new_images;
            state.recreate_swapchain = false;
        }

        let (image_index, suboptimal, acquire_future) =
            match acquire_next_image(state.swapchain.clone(), None)
                .map_err(Validated::unwrap)
            {
                Ok(r) => r,
                Err(VulkanError::OutOfDate) => {
                    state.recreate_swapchain = true;
                    return;
                }
                Err(e) => panic!("failed to acquire next image: {e}"),
            };

        if suboptimal {
            state.recreate_swapchain = true;
        }

        let mut builder = AutoCommandBufferBuilder::primary(
            &state.command_buffer_allocator,
            state.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 0.0, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(
                        state.framebuffers[image_index as usize].clone(),
                    )
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..Default::default()
                },
            )
            .unwrap()
            .set_viewport(0, [state.viewport.clone()].into_iter().collect())
            .unwrap()
            .bind_pipeline_graphics(state.pipeline.clone())
            .unwrap()
            .bind_vertex_buffers(0, state.vertex_buffer.clone())
            .unwrap()
            .draw(state.vertex_buffer.len() as u32, 1, 0, 0)
            .unwrap()
            .end_render_pass(Default::default())
            .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = state.previous_frame_end
            .take()
            .unwrap()
            .join(acquire_future)
            .then_execute(state.queue.clone(), command_buffer)
            .unwrap()
            .then_swapchain_present(
                state.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    state.swapchain.clone(),
                    image_index,
                ),
            )
            .then_signal_fence_and_flush();

        match future.map_err(Validated::unwrap) {
            Ok(future) => {
                state.previous_frame_end = Some(future.boxed());
            }
            Err(VulkanError::OutOfDate) => {
                state.recreate_swapchain = true;
                state.previous_frame_end = Some(sync::now(state.device.clone()).boxed());
            }
            Err(e) => {
                panic!("failed to flush future: {e}");
            }
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

fn window_size_dependent_setup(
    images: &[Arc<Image>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let extent = images[0].extent();
    viewport.extent = [extent[0] as f32, extent[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            ).unwrap()
        })
        .collect::<Vec<_>>()
}