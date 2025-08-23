//
// Camera
//

struct Camera {
	center: vec3f,
	samples_per_pixel: u32,
	pixel00_loc: vec3f,
	max_depth: u32,
	pixel_delta_u: vec3f,
	pixel_delta_v: vec3f,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

//
// Utils
//

const EPSILON: f32 = 1.1920929E-7;

fn near_zero(v: vec3f) -> bool {
	return
		(abs(v.x) < EPSILON) &&
		(abs(v.y) < EPSILON) &&
		(abs(v.z) < EPSILON)
	;
}

//
// RNG
//

var<private> rng_state: u32 = 0xE9BE815E;
const SIGN_EXP: u32 = 0x3F800000;

fn random_u32() -> u32 {
	var x = rng_state;
	x ^= x << 13u;
	x ^= x >> 17u;
	x ^= x << 5u;
	rng_state = x;
	return x;
}

fn random_f32() -> f32 {
	let x = random_u32();
	let bits = (x >> 9) | SIGN_EXP;
	return bitcast<f32>(bits) - 1.0;
}

fn random_f32_range(min: f32, max: f32) -> f32 {
	return min + (max - min) * random_f32();
}

fn random_vector_range(min: f32, max: f32) -> vec3f {
	return vec3f(
		random_f32_range(min, max),
		random_f32_range(min, max),
		random_f32_range(min, max),
	);
}

fn random_unit_vector() -> vec3f {
	loop {
		let p = random_vector_range(-1, 1);
		let length_squared = dot(p, p);
		if EPSILON < length_squared && length_squared <= 1.0 {
			return p / sqrt(length_squared);
		}
	}
	return vec3f(0);
}

//
// Interval
//

struct Interval {
	min: f32,
	max: f32,
}
fn new_interval(min: f32, max: f32) -> Interval {
	var interval: Interval;
	interval.min = min;
	interval.max = max;
	return interval;
}

//
// Ray
//

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
	let offset = vec3f(random_f32() - 0.5, random_f32() - 0.5, 0);
	let pixel_sample = camera.pixel00_loc
		+ ((x + offset.x) * camera.pixel_delta_u)
		+ ((y + offset.y) * camera.pixel_delta_v);

	return new_ray(camera.center, pixel_sample - camera.center);
}

//
// Material
//

struct MaterialReference {
	id: u32
}
struct Material {
	data: vec4f,
	material_type: u32,
}

@group(0) @binding(2)
var<storage> materials: array<Material>;

struct ScatterResult {
	ray: Ray,
	color: vec3f,
}
fn scatter(material_reference: MaterialReference, ray: Ray, hit_record: HitRecord) -> ScatterResult {
	var result: ScatterResult;
	result.color = vec3f(1, 0, 1);
	result.ray = ray;

	let material = materials[material_reference.id];

	switch material.material_type {
		// Lambertian
		case 0: {
			let albedo = material.data.xyz;

			var scatter_direction = hit_record.normal + random_unit_vector();
			if near_zero(scatter_direction) {
				scatter_direction = hit_record.normal;
			}

			result.color = albedo;
			result.ray = new_ray(hit_record.point, scatter_direction);
		}
		// Metal
		case 1: {
			let albedo = material.data.xyz;
			let fuzz = material.data.w;

			let reflected =
				normalize(reflect(ray.direction, hit_record.normal)) +
				(fuzz * random_unit_vector())
			;
			result.color = albedo;
			result.ray = new_ray(hit_record.point, reflected);
		}
		// Dielectric
		case 2: {
			let refraction_index = material.data.x;

			let attenuation = vec3f(1);

			var ri: f32;
			if hit_record.front_face {
				ri = 1.0 / refraction_index;
			} else {
				ri = refraction_index;
			}

			let unit_direction = normalize(ray.direction);
			let cos_theta = min(dot(-unit_direction, hit_record.normal), 1.0);
			let sin_theta = sqrt(1.0 - cos_theta * cos_theta);

			var direction: vec3f;
			if (ri * sin_theta > 1.0) || (reflectance(cos_theta, ri) > random_f32()) {
				direction = reflect(unit_direction, hit_record.normal);
			} else {
				direction = refract2(unit_direction, hit_record.normal, ri);
			}

			result.color = attenuation;
			result.ray = new_ray(hit_record.point, direction);
		}
		default: {}
	}

	return result;
}

fn refract2(uv: vec3f, n: vec3f, etai_over_etat: f32) -> vec3f {
	let cos_theta = min(dot(-uv, n), 1.0);
	let r_out_perp = etai_over_etat * (uv + cos_theta * n);
	let r_out_parallel =
		-sqrt(abs((1.0 - dot(r_out_perp, r_out_perp)))) * n;
	return r_out_perp + r_out_parallel;
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
	var r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
	r0 = r0 * r0;
	return r0 + (1.0 - r0) * pow(1.0 - cosine, 5);
}

//
// Hit record
//

struct HitRecord {
	point: vec3f,
	normal: vec3f,
	t: f32,
	front_face: bool,
	material: MaterialReference,
}
struct HitResult {
	hit: bool,
	record: HitRecord,
}
fn new_hit_record(
	ray: Ray,
	outward_normal: vec3f,
	point: vec3f,
	t: f32,
	material: MaterialReference,
) -> HitRecord {
	let front_face = dot(ray.direction, outward_normal) < 0.0;

	var normal: vec3f;
	if front_face {
		normal = outward_normal;
	} else {
		normal = -outward_normal;
	}

	var hr: HitRecord;
	hr.point = point;
	hr.normal = normal;
	hr.t = t;
	hr.front_face = front_face;
	hr.material = material;
	return hr;
}

//
// Sphere
//

struct Sphere {
	center: vec3f,
	radius: f32,
	material: MaterialReference,
}

@group(0) @binding(1)
var<storage> spheres: array<Sphere>;

//
// Tracing
//

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

@fragment
fn fs_main(@builtin(position) position: vec4f) -> @location(0) vec4f {

	rng_state ^= bitcast<u32>(
		sin(7.289 * position.x + 11.23 * position.y) * 23758.5453
	);

	var color = vec3f();
	for (var i = 0u; i < camera.samples_per_pixel; i++) {
		let ray = get_ray(position.x, position.y);
		color += ray_color(ray);
	}
	color /= f32(camera.samples_per_pixel);
	return vec4f(color, 1.0);
}
