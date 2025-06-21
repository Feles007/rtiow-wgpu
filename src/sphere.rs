use crate::material::MaterialReference;
use glam::Vec3;

#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Sphere {
	pub center: Vec3,
	pub radius: f32,
	pub material: MaterialReference,
	pub _p0: u32,
	pub _p1: u32,
	pub _p2: u32,
}
impl Sphere {
	pub fn new(center: Vec3, radius: f32, material: MaterialReference) -> Self {
		Self {
			center,
			radius,
			material,
			_p0: 0,
			_p1: 0,
			_p2: 0,
		}
	}
}
