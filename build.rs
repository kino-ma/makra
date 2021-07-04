use std::env;

fn main() {
    if let Ok(_) = env::var("RUST_TEST") {
        return;
    }

    let out_dir = "target";
    println!("{}", out_dir);
    let linker_file = env::var("LINKER_FILE").unwrap();

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=wasm_binary");
    println!("cargo:rerun-if-changed={}", linker_file);
    println!("cargo:rerun-if-changed=build.rs");
}
