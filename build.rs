use std::{env, process::Command};

fn main() {
    if let Ok(_) = env::var("RUST_TEST") {
        return;
    }

    let out_dir = "target";
    println!("{}", out_dir);
    let linker_file = env::var("LINKER_FILE").unwrap();

    use std::fs::File;
    let wasm_bin_path = "compile/wasm-binaries/test.wasm";
    let wasm_bin = File::open(wasm_bin_path).expect("failed to open wasm bin in build.rs");
    let size = wasm_bin.metadata().unwrap().len();
    println!("cargo:rustc-env=WASM_SIZE={}", size);

    if size % 8 != 0 {
        let aligned_size = (size / 8 + 1) * 8;
        Command::new("truncate")
            .arg("--size")
            .arg(format!("{}", aligned_size))
            .arg(wasm_bin_path)
            .spawn()
            .expect("failed to execute `truncate'");
    }

    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-lib=static=wasm_binary");
    println!("cargo:rerun-if-changed={}", linker_file);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=compile/wasm-binaries/test.wat");
}
