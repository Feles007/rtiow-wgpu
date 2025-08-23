import package::{ray::Ray, material::MaterialReference};

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