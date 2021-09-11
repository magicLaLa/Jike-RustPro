use std::sync::Arc;

fn main() {
  let arr = Arc::new(vec![1]);

  let arr2 = arr.clone();

  std::thread::spawn(move|| {
    let tt = arr.get(0).unwrap();
    println!("{:?}", tt);
  });

  println!("out in {:?}", arr2.get(0).unwrap());
}