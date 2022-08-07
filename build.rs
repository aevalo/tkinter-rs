extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  println!("cargo:rustc-link-lib=bz2");
  println!("cargo:rerun-if-changed=build/bz2-wrapper.h");

  let mut bindings = bindgen::Builder::default()
    .header("build/bz2-wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings for bz2");

  bindings
    .write_to_file(out_path.join("bz2-bindings.rs"))
    .expect("Couldn't write bindings for bz2!");

  /*
  println!("cargo:rustc-link-lib=tcl8.6");
  println!("cargo:rustc-link-lib=tclstub8.6");
  println!("cargo:rerun-if-changed=build/tcl-wrapper.h");

  bindings = bindgen::Builder::default()
    .header("build/tcl-wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings for tcl");

  bindings
    .write_to_file(out_path.join("tcl-bindings.rs"))
    .expect("Couldn't write bindings for tcl!");

  println!("cargo:rustc-link-lib=tk8.6");
  println!("cargo:rustc-link-lib=tkstub8.6");
  println!("cargo:rerun-if-changed=build/tk-wrapper.h");

  bindings = bindgen::Builder::default()
    .header("build/tk-wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings for tk");

  bindings
    .write_to_file(out_path.join("tk-bindings.rs"))
    .expect("Couldn't write bindings for tk!");
  */
}
