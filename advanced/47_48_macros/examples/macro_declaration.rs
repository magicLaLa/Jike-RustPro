/// 声明宏
#[macro_export]
macro_rules! my_vec {
  // 不带任何参数的
  () => {
    std::vec::Vec::new()
  };
  // 处理 my_vec![1,2,3,4]
  ($($el:expr), *) => ({
    let mut v = std::vec::Vec::new();
    $(v.push($el);)*
    v
  });
  // 处理 my_vec!{[0; 10]}
  ($el:expr; $n:expr) => {
    std::vec::from_elem($el, $n)
  }
}

fn main() {
  let mut v = my_vec![];
  v.push(1);
  // 不同的调用方式
  let _v = my_vec!(1,3,4,5);
  let _v = my_vec![1, 2, 3, 4];
  let v = my_vec! { 1, 2, 3, 4 };
  println!("{:?}", v);
  let v = my_vec![1; 10];
  println!("{:?}", v);
}