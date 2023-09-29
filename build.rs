use libninja::generate_library_using_spec_at_path;
use ln_core::{Language, LibraryOptions, OutputOptions};
pub fn main() {
    println!("cargo:rerun-if-changed=petstore.yaml");
    let schema_path = std::env::current_dir().unwrap().join("petstore.yaml");
    let dest_path = std::env::current_dir().unwrap();
    let options = OutputOptions {
        library_options: LibraryOptions::new("petstore", Language::Rust),
        qualified_github_repo: String::from("prabhpreet/libninja-all-of-example"),
        dest_path: dest_path.clone(),
    };
    generate_library_using_spec_at_path(schema_path.as_path(), options).unwrap();
    //Remove [workspaces] from Cargo.toml
    let mut cargo_toml = std::fs::read_to_string(dest_path.join("Cargo.toml")).unwrap();
    cargo_toml = cargo_toml.replace("[workspace]\n", "");
    std::fs::write(dest_path.join("Cargo.toml"), cargo_toml).unwrap();
}
