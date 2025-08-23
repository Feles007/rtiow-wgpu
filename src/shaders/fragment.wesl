import package::{
	rng,
	camera::Camera,
	material::Material,
	sphere::Sphere,
	ray::get_ray,
	tracing::ray_color,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(0) @binding(1)
var<storage> spheres: array<Sphere>;

@group(0) @binding(2)
var<storage> materials: array<Material>;

@fragment
fn fs_main(@builtin(position) position: vec4f) -> @location(0) vec4f {

	rng::seed(position.xy);

	var color = vec3f();
	for (var i = 0u; i < camera.samples_per_pixel; i++) {
		let ray = get_ray(position.x, position.y);
		color += ray_color(ray);
	}
	color /= f32(camera.samples_per_pixel);
	return vec4f(color, 1.0);
}
