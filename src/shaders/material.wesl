import package::{rng, ray::{Ray, new_ray}, hit_record::HitRecord, util::near_zero};
import package::fragment::materials;

struct MaterialReference {
	id: u32
}
struct Material {
	data: vec4f,
	material_type: u32,
}


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

			var scatter_direction = hit_record.normal + rng::unit_vector();
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
				(fuzz * rng::unit_vector())
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
			if (ri * sin_theta > 1.0) || (reflectance(cos_theta, ri) > rng::float()) {
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