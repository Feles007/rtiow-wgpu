mod app;
mod camera;
mod material;
mod sphere;
mod state;
mod world;

use crate::app::App;
use crate::camera::CameraParameters;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::world::World;
use glam::vec3;
use std::f32::consts::FRAC_PI_2;
use winit::event_loop::{ControlFlow, EventLoop};

fn main() {
	env_logger::init();
	let event_loop = EventLoop::new().unwrap();
	event_loop.set_control_flow(ControlFlow::Poll);
	let mut app = App::new();
	event_loop.run_app(&mut app).unwrap();
}

mod rng {
	use glam::{vec3, Vec3};
	use std::sync::atomic::{AtomicU32, Ordering};

	static RNG_STATE: AtomicU32 = AtomicU32::new(0xE9BE815E);

	pub fn f32() -> f32 {
		const SIGN_EXP: u32 = 0x3F800000;

		let mut x = RNG_STATE.load(Ordering::Relaxed);
		x ^= x << 13;
		x ^= x >> 17;
		x ^= x << 5;
		RNG_STATE.store(x, Ordering::Relaxed);
		f32::from_bits((x >> 9) | SIGN_EXP) - 1.0
	}

	pub fn f32_range(min: f32, max: f32) -> f32 {
		min + (max - min) * f32()
	}
	pub fn vector() -> Vec3 {
		vec3(f32(), f32(), f32())
	}
}

fn setup() -> (World, CameraParameters) {
	let world = {
		let mut world = World::new();

		let ground_material = world.add_material(Material::lambertian(vec3(0.5, 0.5, 0.5)));
		world.add_sphere(Sphere::new(vec3(0.0, -1000.0, 0.0), 1000.0, ground_material));

		for a in -11..11 {
			for b in -11..11 {
				let a = a as f32;
				let b = b as f32;

				let choose_mat = rng::f32();
				let center = vec3(a + 0.9 * rng::f32(), 0.2, b + 0.9 * rng::f32());

				if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
					let sphere_material;

					if choose_mat < 0.8 {
						// diffuse
						let albedo = rng::vector() * rng::vector();
						sphere_material = world.add_material(Material::lambertian(albedo));
						world.add_sphere(Sphere::new(center, 0.2, sphere_material));
					} else if choose_mat < 0.95 {
						// metal
						let albedo = rng::vector() * rng::vector();
						let fuzz = rng::f32_range(0.0, 0.5);
						sphere_material = world.add_material(Material::metal(albedo, fuzz));
						world.add_sphere(Sphere::new(center, 0.2, sphere_material));
					} else {
						// glass
						sphere_material = world.add_material(Material::dielectric(1.5));
						world.add_sphere(Sphere::new(center, 0.2, sphere_material));
					}
				}
			}
		}

		let material1 = world.add_material(Material::dielectric(1.5));
		world.add_sphere(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, material1));

		let material2 = world.add_material(Material::lambertian(vec3(0.4, 0.2, 0.1)));
		world.add_sphere(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, material2));

		let material3 = world.add_material(Material::metal(vec3(0.7, 0.6, 0.5), 0.0));
		world.add_sphere(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3));

		world
	};

	let pitch = 0.0;
	let yaw = FRAC_PI_2;

	let camera_parameters = CameraParameters {
		samples_per_pixel: 3,
		max_depth: 5,
		fov: 20.0,
		location: vec3(13.0, 2.0, 3.0),
		pitch,
		yaw,
	};

	(world, camera_parameters)
}
