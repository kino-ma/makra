use std::env;

fn main() {
    if let Ok(_) = env::var("RUST_TEST") {
        return
    }

    let linker_file = env::var("LINKER_FILE").unwrap();

    println!("cargo:rerun-if-changed={}", linker_file);
    println!("cargo:rerun-if-changed=build.rs");
}