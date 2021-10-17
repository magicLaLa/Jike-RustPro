use std::{panic, result};

fn main() {
  let result = panic::catch_unwind(|| {
    println!("heoole!");
  });
  assert!(result.is_ok());
  let result = panic::catch_unwind(|| {
    panic!("oh no!");
  });
  println!("panic captureed: {:#?}", result);
}