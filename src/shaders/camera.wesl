struct Camera {
	center: vec3f,
	samples_per_pixel: u32,
	pixel00_loc: vec3f,
	max_depth: u32,
	pixel_delta_u: vec3f,
	pixel_delta_v: vec3f,
}