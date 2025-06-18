use pretty_assertions::assert_eq;
use wgsl_to_wgpu::{Module, TypePath};

fn demangle_underscore(name: &str) -> TypePath {
    // Preprocessors that support modules mangle absolute paths.
    // Use a very basic mangling scheme that assumes no '_' in the identifier name.
    // This allows testing the module logic without needing extra dependencies.
    // a_b_C -> a::b::C
    let components: Vec<_> = name.split("_").collect();
    let (name, parents) = components.split_last().unwrap();
    TypePath {
        parents: parents.into_iter().map(|p| p.to_string()).collect(),
        name: name.to_string(),
    }
}

#[test]
fn single_module() {
    let mut root = Module::default();
    let options = wgsl_to_wgpu::WriteOptions {
        rustfmt: true,
        ..Default::default()
    };
    root.add_shader_module(
        include_str!("wgsl/modules.wgsl"),
        None,
        options,
        TypePath {
            parents: Vec::new(),
            name: String::new(),
        },
        demangle_underscore,
    )
    .unwrap();

    let output = root.to_generated_bindings(options);
    assert_eq!(include_str!("output/modules.rs"), output);
}
