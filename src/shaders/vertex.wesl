struct VertexOutput {
	@builtin(position) clip_position: vec4f,
};

@vertex
fn vs_main(
	@builtin(vertex_index) vertex_index: u32,
) -> @builtin(position) vec4f {
	const clip_position = array(
		// Tri 1
		vec2(-1.0,  3.0),
		vec2(-1.0, -1.0),
		vec2( 3.0, -1.0),
	);
	return vec4f(clip_position[vertex_index], 0.0, 1.0);
}