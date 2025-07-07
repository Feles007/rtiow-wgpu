use glam::{vec3, Vec3};
use wgpu::{BindingResource, Buffer, BufferAddress, BufferDescriptor, BufferUsages, Device, Queue};

pub struct CameraParameters {
	pub samples_per_pixel: u32,
	pub max_depth: u32,
	pub fov: f32,
	pub location: Vec3,
	pub pitch: f32,
	pub yaw: f32,
}
pub struct Camera {
	pub parameters: CameraParameters,
	pub width: u32,
	pub height: u32,
	buffer: Buffer,
}
impl Camera {
	pub fn new(device: &Device, parameters: CameraParameters, width: u32, height: u32) -> Self {
		Self {
			parameters,
			width,
			height,
			buffer: device.create_buffer(&BufferDescriptor {
				label: Some("Camera Uniform Buffer"),
				size: size_of::<CameraUniform>() as BufferAddress,
				usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
				mapped_at_creation: false,
			}),
		}
	}
	pub fn update_buffer(&self, queue: &Queue) {
		let camera_uniform = {
			let p = &self.parameters;

			let center = p.location;
			let direction = make_look(p.pitch, p.yaw);

			let focal_length = direction.length();
			let theta = p.fov.to_radians();
			let h = (theta / 2.0).tan();
			let viewport_height = 2.0 * h * focal_length;
			let viewport_width = viewport_height * (self.width as f32 / self.height as f32);

			let up_vector = vec3(0.0, 1.0, 0.0);

			let w = direction.normalize();
			let u = up_vector.cross(w).normalize();
			let v = w.cross(u);

			let viewport_u = viewport_width * u;
			let viewport_v = viewport_height * -v;

			let pixel_delta_u = viewport_u / (self.width as f32);
			let pixel_delta_v = viewport_v / (self.height as f32);

			let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
			let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

			CameraUniform {
				center,
				samples_per_pixel: self.parameters.samples_per_pixel,
				pixel00_loc,
				max_depth: self.parameters.max_depth,
				pixel_delta_u,
				_p2: 0,
				pixel_delta_v,
				_p3: 0,
			}
		};
		queue.write_buffer(&self.buffer, 0, bytemuck::bytes_of(&camera_uniform));
	}
	pub fn binding(&self) -> BindingResource {
		self.buffer.as_entire_binding()
	}
}
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraUniform {
	center: Vec3,
	samples_per_pixel: u32,
	pixel00_loc: Vec3,
	max_depth: u32,
	pixel_delta_u: Vec3,
	_p2: u32,
	pixel_delta_v: Vec3,
	_p3: u32,
}
pub fn make_look(pitch: f32, yaw: f32) -> Vec3 {
	vec3(yaw.sin() * pitch.cos(), pitch.sin(), yaw.cos() * pitch.cos())
}
