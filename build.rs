// build.rs
use std::env;
use std::path::PathBuf;

fn main() {
    println!(r"cargo:rerun-if-changed=wrapper.h");
    println!(r"cargo:rustc-link-lib=tsk");

    #[cfg(feature = "windows")]
    windows_setup();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .allowlist_type("TSK_.*")
        .allowlist_function("tsk_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

#[cfg(feature = "windows")]
fn windows_setup() {
    // Windows-specific setup code...
}