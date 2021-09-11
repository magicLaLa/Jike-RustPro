
// fn main() {
//   let mut arr = vec![1, 2, 3];
//   // cache the last item
//   let last = arr.last();
//   // consume previously stored last item
//   println!("last: {:?}", last);
//   arr.push(4);
// }

// fn main() {
//   let mut arr = vec![1, 2, 3];
//   arr.push(4);
//   // cache the last item
//   let last = arr.last();
//   // consume previously stored last item
//   println!("last: {:?}", last);
// }

fn main() {
  let mut arr = vec![1, 2, 3];
  let len: usize = arr.len() - 1;
  // cache the last item
  let last = arr[len -  1];
  arr.push(4);
  // consume previously stored last item
  println!("last: {:?}", last);
}