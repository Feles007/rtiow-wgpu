const EPSILON: f32 = 1.1920929E-7;

fn near_zero(v: vec3f) -> bool {
	return
		(abs(v.x) < EPSILON) &&
		(abs(v.y) < EPSILON) &&
		(abs(v.z) < EPSILON)
	;
}