use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let kernel = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("cargo:rerun-if-changed={}", kernel.join("src").display());
    println!("cargo:rustc-link-arg=-T{}", out_dir.join("linker.ld").display());
}
