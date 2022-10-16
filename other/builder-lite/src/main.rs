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

trait Processor {
    fn compute(&self, x: i64, y: i64) -> i64;
}

struct Risc {}

impl Processor for Risc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x + y
    }
}

struct Cisc {}

impl Processor for Cisc {
    fn compute(&self, x: i64, y: i64) -> i64 {
        x + y
    }
}

/// 静态分发，编译时确定大小
fn process(processor: impl Processor, x: i64) {
    let result = processor.compute(x, 42);
    println!("result: {}", result);
}
/// 动态分发，编译时只根据 指针来，因为指针大小是确定的，只有在运行时找到一对应方法或者函数调用
fn process2(processor: Box<dyn Processor>, x: i64) {
    let result = processor.compute(x, 42);
    println!("result: {}", result);
}

fn main() {
    let shape = Shape::new(1.4).with_test(4.6).with_postion(vec![2, 4]);
    Shape::to_sss();
    println!("shape {:#?}", shape);

    let processor1 = Risc {};
    let processor2 = Cisc {};

    process(processor1, 1);
    process(processor2, 2);

    let processors: Vec<Box<dyn Processor>> = vec![
        Box::new(Risc {}), Box::new(Cisc {})
    ];

    for processor in processors {
        process2(processor, 1);
    }
}
