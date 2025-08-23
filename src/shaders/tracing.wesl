import package::{
	sphere::Sphere,
	ray::{Ray, ray_at},
	interval::{Interval, new_interval},
	hit_record::{HitResult, new_hit_record},
	material::scatter,
	fragment::spheres,
};

fn hit_sphere(sphere: Sphere, ray: Ray, interval: Interval) -> HitResult {
	var result: HitResult;
	result.hit = false;

	let oc = sphere.center - ray.origin;
	let a = dot(ray.direction, ray.direction);
	let h = dot(ray.direction, oc);
	let c = dot(oc, oc) - sphere.radius * sphere.radius;

	let discriminant = h * h - a * c;
	if discriminant < 0.0 {
		return result;
	}

	let sqrtd = sqrt(discriminant);

	var root = (h - sqrtd) / a;
	if root <= interval.min || interval.max <= root {
		root = (h + sqrtd) / a;
		if root <= interval.min || interval.max <= root {
			return result;
		}
	}

	let point = ray_at(ray, root);
	let normal = (point - sphere.center) / sphere.radius;

	result.record = new_hit_record(ray, normal, point, root, sphere.material);
	result.hit = true;
	return result;
}
fn hit_world(ray: Ray, interval: Interval) -> HitResult {
	var result: HitResult;
	result.hit = false;

	var closest_so_far = interval.max;

	let sphere_count = arrayLength(&spheres);
	for (var i = 0u; i < sphere_count; i++) {
		let sphere_hit_result = hit_sphere(spheres[i], ray, new_interval(interval.min, closest_so_far));
		if sphere_hit_result.hit {
			let record = sphere_hit_result.record;
			closest_so_far = record.t;
			result.hit = true;
			result.record = record;
		}
	}

	return result;
}
fn ray_color(ray: Ray) -> vec3f {
	var color = vec3f(0);
	var first_color = true;

	var current_ray = ray;

	for (var i = 0u; i < camera.max_depth; i++) {
		let interval = new_interval(0.0001, 10000000.0);

		let hit_result = hit_world(current_ray, interval);
		if !hit_result.hit { break; }

		let scatter_result = scatter(hit_result.record.material, current_ray, hit_result.record);

		if first_color {
			color = scatter_result.color;
			first_color = false;
		} else {
			color *= scatter_result.color;
		}

		current_ray = scatter_result.ray;
	}

	let bgc = background_color(current_ray);

	if first_color {
		return bgc;
	} else {
		return color * bgc;
	}
}
fn background_color(ray: Ray) -> vec3f {
	let unit_direction = normalize(ray.direction);
	let a = 0.5 * (unit_direction.y + 1.0);
	return (1.0 - a) * vec3f(1) + a * vec3f(0.5, 0.7, 1.0);
}