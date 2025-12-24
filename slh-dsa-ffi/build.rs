use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = PathBuf::from(&crate_dir).join("include");
    let header_path = out_dir.join("slh_dsa.h");

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_header("#include <stdbool.h>\n#include <stddef.h>\n#include <stdint.h>\n")
        .with_include_guard("SLH_DSA_H")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(header_path);

    println!("cargo:rerun-if-changed=src/lib.rs");
}
