import package::rng;

struct Ray {
	origin: vec3f,
	direction: vec3f,
}
fn new_ray(origin: vec3f, direction: vec3f) -> Ray {
	var ray: Ray;
	ray.origin = origin;
	ray.direction = direction;
	return ray;
}
fn ray_at(ray: Ray, t: f32) -> vec3f {
	return ray.origin + t * ray.direction;
}
fn get_ray(x: f32, y: f32) -> Ray {
	let offset = vec3f(rng::float() - 0.5, rng::float() - 0.5, 0);
	let pixel_sample = camera.pixel00_loc
		+ ((x + offset.x) * camera.pixel_delta_u)
		+ ((y + offset.y) * camera.pixel_delta_v);

	return new_ray(camera.center, pixel_sample - camera.center);
}