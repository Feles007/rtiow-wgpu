use glam::{vec4, Vec3, Vec4};

#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct MaterialReference {
	id: u32,
}
impl MaterialReference {
	pub fn new(id: u32) -> Self {
		Self { id }
	}
}

#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Material {
	data: Vec4,
	material_type: u32,
	_p0: u32,
	_p1: u32,
	_p2: u32,
}
impl Material {
	pub fn lambertian(albedo: Vec3) -> Self {
		Self {
			data: albedo.extend(0.0),
			material_type: 0,
			_p0: 0,
			_p1: 0,
			_p2: 0,
		}
	}
	pub fn metal(albedo: Vec3, fuzz: f32) -> Self {
		Self {
			data: albedo.extend(fuzz),
			material_type: 1,
			_p0: 0,
			_p1: 0,
			_p2: 0,
		}
	}
	pub fn dielectric(refraction_index: f32) -> Self {
		Self {
			data: vec4(refraction_index, 0.0, 0.0, 0.0),
			material_type: 2,
			_p0: 0,
			_p1: 0,
			_p2: 0,
		}
	}
}
