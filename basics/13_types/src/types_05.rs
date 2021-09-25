use std::ops::Add;

#[derive(Debug)]
struct Complex {
  real: f64,
  imageine: f64,
}

impl Complex {
    pub fn new(real: f64, imageine: f64) -> Self {
      Self {
        real,
        imageine,
      }
    }
}

// 对 Complex 类型的实现
impl Add for Complex {
  type Output = Self;

  // 注意 add 第一个参数是 self，会移动所有权
  fn add(self, rhs: Self) -> Self::Output {
      let real = self.real + rhs.real;
      let imageine = self.imageine + rhs.imageine;
      Self::new(real, imageine)
  }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
      let real = self.real + rhs.real;
      let imageine = self.imageine + rhs.imageine;
      Complex::new(real, imageine)
    }
}

impl Add<f64> for &Complex {
  type Output = Complex;

  // 注意 add 第一个参数是 self，会移动所有权
  fn add(self, rhs: f64) -> Self::Output {
      let real = self.real + rhs;
      Complex::new(real, self.imageine)
  }
}

fn main() {
  let c1 = Complex::new(1.0, 1f64);
  let c2 = Complex::new(2 as f64, 3.0);
  println!("{:?}", &c1 + &c2);
  println!("{:?}", &c1 + 5.0);
  println!("{:?}", c1 + c2);
}