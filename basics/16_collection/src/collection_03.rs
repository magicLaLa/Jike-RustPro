fn main() {
  let result = vec![1, 2, 3, 4]
    .iter()
    .map(|v| v*v)
    .filter(|v| *v < 16)
    .take(1)
    .collect::<Vec<_>>();

    println!("{:?}", result);
}