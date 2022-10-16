/// bolg 地址：https://matklad.github.io/2022/05/29/builder-lite.html

#[derive(Debug)]
pub struct Shape {
    position: Vec<usize>,
    test: f64,
}

impl Shape {
    pub fn new(test: f64) -> Self {
        Self {
            position: Vec::new(),
            test,
        }
    }

    pub fn to_sss() {
        println!("to_sss");
    }

    pub fn with_test(mut self, test: f64) -> Shape {
        self.test = test;
        self
    }

    /// 新增方法
    pub fn with_postion(mut self, position: Vec<usize>) -> Self {
        self.position = position;
        self
    }
}

fn main() {
    let shape = Shape::new(1.4).with_test(4.6).with_postion(vec![2, 4]);
    Shape::to_sss();
    println!("shape {:#?}", shape);
}
