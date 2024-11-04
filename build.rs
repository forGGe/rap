use std::{env, path::PathBuf};

use cc;

fn main() {
    println!("cargo::rerun-if-changed=srcc/slist.c");
    println!("cargo::rerun-if-changed=srcc/slist.h");
    println!("cargo::rustc-link-lib=slist");

    cc::Build::new().file("srcc/slist.c").compile("slist");

    let bindings = bindgen::Builder::default()
        .header("srcc/slist.h")
        .generate_inline_functions(true)
        .generate()
        .expect("failed to generate bindings");

    let outpath = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(outpath.join("slistc_bindings.rs"))
        .expect("failed to write bindings");
}
