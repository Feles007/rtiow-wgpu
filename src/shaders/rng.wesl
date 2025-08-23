var<private> rng_state: u32 = 0xE9BE815E;
const SIGN_EXP: u32 = 0x3F800000;

fn seed(position: vec2f) {
	rng_state ^= bitcast<u32>(
		sin(7.289 * position.x + 11.23 * position.y) * 23758.5453
	);
}

fn bits() -> u32 {
	var x = rng_state;
	x ^= x << 13u;
	x ^= x >> 17u;
	x ^= x << 5u;
	rng_state = x;
	return x;
}

fn float() -> f32 {
	let x = bits();
	let bits = (x >> 9) | SIGN_EXP;
	return bitcast<f32>(bits) - 1.0;
}

fn float_range(min: f32, max: f32) -> f32 {
	return min + (max - min) * float();
}

fn vector_range(min: f32, max: f32) -> vec3f {
	return vec3f(
		float_range(min, max),
		float_range(min, max),
		float_range(min, max),
	);
}

fn unit_vector() -> vec3f {
	loop {
		let p = vector_range(-1, 1);
		let length_squared = dot(p, p);
		if EPSILON < length_squared && length_squared <= 1.0 {
			return p / sqrt(length_squared);
		}
	}
	return vec3f(0);
}