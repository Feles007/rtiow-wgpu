use crate::material::{Material, MaterialReference};
use crate::sphere::Sphere;

pub struct World {
	materials: Vec<Material>,
	spheres: Vec<Sphere>,
}
impl World {
	pub fn new() -> Self {
		Self {
			materials: Vec::new(),
			spheres: Vec::new(),
		}
	}
	pub fn add_material(&mut self, material: Material) -> MaterialReference {
		let mr = MaterialReference::new(u32::try_from(self.materials.len()).unwrap());
		self.materials.push(material);
		mr
	}
	pub fn add_sphere(&mut self, sphere: Sphere) {
		self.spheres.push(sphere);
	}
	pub fn materials(&self) -> &[Material] {
		&self.materials
	}
	pub fn spheres(&self) -> &[Sphere] {
		&self.spheres
	}
}
