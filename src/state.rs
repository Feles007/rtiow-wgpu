use crate::app::ControlMap;
use crate::camera::{Camera, CameraParameters};
use crate::world::World;
use std::sync::Arc;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{
	Backends, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
	BindingType, BlendComponent, BlendState, BufferBindingType, BufferUsages, Color, ColorTargetState, ColorWrites,
	CompositeAlphaMode, Device, DeviceDescriptor, Face, FragmentState, FrontFace, Instance, InstanceDescriptor, LoadOp,
	MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PresentMode, PrimitiveState,
	PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
	RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp,
	Surface, SurfaceConfiguration, TextureFormat, TextureUsages, TextureViewDescriptor, VertexState,
};
use winit::dpi::PhysicalSize;
use winit::window::{CursorGrabMode, Window};

pub struct State {
	window: Arc<Window>,
	device: Device,
	queue: Queue,
	size: PhysicalSize<u32>,
	surface: Surface<'static>,
	surface_format: TextureFormat,
	pipeline: RenderPipeline,
	camera: Camera,
	bind_group: BindGroup,
	is_mouse_focused: bool,
}

impl State {
	pub async fn new(window: Arc<Window>, camera_parameters: CameraParameters, world: &World) -> State {
		let instance = Instance::new(&InstanceDescriptor {
			backends: Backends::VULKAN,
			..Default::default()
		});
		let adapter = instance
			.request_adapter(&RequestAdapterOptions::default())
			.await
			.unwrap();
		let (device, queue) = adapter.request_device(&DeviceDescriptor::default()).await.unwrap();

		let size = window.inner_size();

		let surface = instance.create_surface(window.clone()).unwrap();
		let cap = surface.get_capabilities(&adapter);
		let surface_format = cap.formats[0];

		let camera = Camera::new(&device, camera_parameters, size.width, size.height);

		let sphere_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Sphere Buffer"),
			contents: bytemuck::cast_slice(world.spheres()),
			usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
		});

		let material_buffer = device.create_buffer_init(&BufferInitDescriptor {
			label: Some("Material Buffer"),
			contents: bytemuck::cast_slice(world.materials()),
			usage: BufferUsages::STORAGE | BufferUsages::COPY_DST,
		});

		let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
			label: None,
			entries: &[
				BindGroupLayoutEntry {
					binding: 0,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Buffer {
						ty: BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 1,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Buffer {
						ty: BufferBindingType::Storage { read_only: true },
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				},
				BindGroupLayoutEntry {
					binding: 2,
					visibility: ShaderStages::FRAGMENT,
					ty: BindingType::Buffer {
						ty: BufferBindingType::Storage { read_only: true },
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				},
			],
		});

		let bind_group = device.create_bind_group(&BindGroupDescriptor {
			layout: &bind_group_layout,
			entries: &[
				BindGroupEntry {
					binding: 0,
					resource: camera.binding(),
				},
				BindGroupEntry {
					binding: 1,
					resource: sphere_buffer.as_entire_binding(),
				},
				BindGroupEntry {
					binding: 2,
					resource: material_buffer.as_entire_binding(),
				},
			],
			label: Some("Bind Group"),
		});

		let pipeline = {
			let shader = device.create_shader_module(ShaderModuleDescriptor {
				label: Some("Shader"),
				source: ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
			});

			let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
				label: Some("Render Pipeline Layout"),
				bind_group_layouts: &[&bind_group_layout],
				push_constant_ranges: &[],
			});

			let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
				label: Some("Render Pipeline"),
				layout: Some(&render_pipeline_layout),
				vertex: VertexState {
					module: &shader,
					entry_point: Some("vs_main"),
					buffers: &[],
					compilation_options: Default::default(),
				},
				fragment: Some(FragmentState {
					module: &shader,
					entry_point: Some("fs_main"),
					targets: &[Some(ColorTargetState {
						format: surface_format,
						blend: Some(BlendState {
							color: BlendComponent::REPLACE,
							alpha: BlendComponent::REPLACE,
						}),
						write_mask: ColorWrites::ALL,
					})],
					compilation_options: Default::default(),
				}),
				primitive: PrimitiveState {
					topology: PrimitiveTopology::TriangleList,
					strip_index_format: None,
					front_face: FrontFace::Ccw,
					cull_mode: Some(Face::Back),
					// Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
					// or Features::POLYGON_MODE_POINT
					polygon_mode: PolygonMode::Fill,
					// Requires Features::DEPTH_CLIP_CONTROL
					unclipped_depth: false,
					// Requires Features::CONSERVATIVE_RASTERIZATION
					conservative: false,
				},
				depth_stencil: None,
				multisample: MultisampleState {
					count: 1,
					mask: !0,
					alpha_to_coverage_enabled: false,
				},
				// If the pipeline will be used with a multiview render pass, this
				// indicates how many array layers the attachments will have.
				multiview: None,
				// Useful for optimizing shader compilation on Android
				cache: None,
			});

			render_pipeline
		};

		let state = State {
			window,
			device,
			queue,
			size,
			surface,
			surface_format,
			pipeline,
			camera,
			bind_group,
			is_mouse_focused: false,
		};

		state.configure_surface();

		state
	}

	pub fn get_window(&self) -> &Window {
		&self.window
	}

	fn configure_surface(&self) {
		let surface_config = SurfaceConfiguration {
			usage: TextureUsages::RENDER_ATTACHMENT,
			format: self.surface_format,
			view_formats: vec![self.surface_format.add_srgb_suffix()],
			alpha_mode: CompositeAlphaMode::Auto,
			width: self.size.width,
			height: self.size.height,
			desired_maximum_frame_latency: 2,
			present_mode: PresentMode::Immediate,
		};
		self.surface.configure(&self.device, &surface_config);
	}

	pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
		if self.size == new_size {
			return;
		}

		self.size = new_size;

		if self.size.width > 0 && self.size.height > 0 {
			self.configure_surface();
			self.camera.update_buffer(&self.queue);
		}
	}

	pub fn update(&mut self, control_map: &mut ControlMap, delta_time: f32) {}

	pub fn render(&mut self) {
		if self.size.width == 0 || self.size.height == 0 {
			return;
		}

		let surface_texture = self.surface.get_current_texture().unwrap();

		let texture_view = surface_texture.texture.create_view(&TextureViewDescriptor {
			format: Some(self.surface_format.add_srgb_suffix()),
			..Default::default()
		});

		let mut encoder = self.device.create_command_encoder(&Default::default());

		{
			let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
				label: None,
				color_attachments: &[Some(RenderPassColorAttachment {
					view: &texture_view,
					resolve_target: None,
					ops: Operations {
						load: LoadOp::Clear(Color::GREEN),
						store: StoreOp::Store,
					},
				})],
				depth_stencil_attachment: None,
				timestamp_writes: None,
				occlusion_query_set: None,
			});

			render_pass.set_bind_group(0, &self.bind_group, &[]);

			render_pass.set_pipeline(&self.pipeline);
			render_pass.draw(0..3, 0..1);
		}

		self.queue.submit([encoder.finish()]);
		self.window.pre_present_notify();
		surface_texture.present();
	}

	pub fn focus(&mut self) {
		let result = self.window.set_cursor_grab(CursorGrabMode::Confined);
		match result {
			Ok(_) => {},
			Err(_) => {
				self.window.set_cursor_grab(CursorGrabMode::Locked).unwrap();
			},
		}
		self.window.set_cursor_visible(false);
		self.is_mouse_focused = true;
	}
	pub fn unfocus(&mut self) {
		self.window.set_cursor_grab(CursorGrabMode::None).unwrap();
		self.window.set_cursor_visible(true);
		self.is_mouse_focused = false;
	}
	pub fn is_mouse_focused(&self) -> bool {
		self.is_mouse_focused
	}
	pub fn request_redraw(&self) {
		self.window.request_redraw();
	}
}
