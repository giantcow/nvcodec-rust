use std::path::PathBuf;

fn main() {
    // Newer versions of the cuda-toolkit installer use these locations. Note that they may be
    // symlinks
    println!("cargo:rustc-link-search=/usr/lib64/");
    println!("cargo:rustc-link-search=/usr/local/cuda/");

    println!("cargo:rustc-link-lib=nvidia-encode");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Disable generating bindings for the GUID definitions as bindgen thinks/assumes they're external functions
        // See: https://github.com/rust-lang/rust-bindgen/issues/1266
        .blocklist_item("NV_ENC_.*_GUID")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
