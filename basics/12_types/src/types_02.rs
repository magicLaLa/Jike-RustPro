use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::{Path};

struct MyReader<R> {
  redner: R,
  buf: String,
}

impl<R> MyReader<R> {
    pub fn new(reader: R) -> Self {
      Self {
        redner: reader,
        buf: String::with_capacity(1024),
      }
    }
}

impl<R> MyReader<R> where R: Read  {
    pub fn prcess(&mut self) -> Result<usize> {
      self.redner.read_to_string(&mut self.buf)
    }
}

fn main() {
  let path = Path::new("./types_02.rs");
  println!("path: {:?}", path);
  let f = File::open(path).unwrap();
  let mut reader = MyReader::new(BufReader::new(f));

  let size = reader.prcess().unwrap();
  println!("total size read: {}", size);
}