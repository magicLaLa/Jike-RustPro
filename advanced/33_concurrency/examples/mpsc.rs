use std::sync::{
  atomic,
  mpsc,
};
use std::thread;
use std::time::Duration;

fn main() {
  let (a1, a1t) = mpsc::channel();
  let (a2, a2t) = mpsc::channel();

  let thread1 = thread::spawn(move || {
    a1.send("hello world".to_string());
    for re in a2t {
        println!("{}\n", re);
        thread::sleep(Duration::from_secs(1));
        a1.send("hello world".to_string());
    }
  });

  let thread2 = thread::spawn(move || {
    for a1re in a1t {
        println!("{}\n", a1re);
        thread::sleep(Duration::from_secs(1));
        a2.send("goodbye".to_string());
    }
  });

  thread::sleep(Duration::from_secs(10));
}