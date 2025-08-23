use wesl::Wesl;

fn main() {
	let compiler = Wesl::new("src/shaders");
	compiler.build_artifact(&"package::fragment".parse().unwrap(), "fragment");
	compiler.build_artifact(&"package::vertex".parse().unwrap(), "vertex");
}
