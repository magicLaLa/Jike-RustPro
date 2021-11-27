use std::{env};

fn main() {
  println!("cargo:rusts-link-lib=bz2");
  println!("cargo:rerun-if-changed=wrapper.h");

  let bindings = bindgen::Builder::default()
    .header("wrapper.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

  let path = env::current_dir().unwrap().join("src/bindings.rs");

  bindings
    .write_to_file(path)
    .expect("Failed to write bindings");
}