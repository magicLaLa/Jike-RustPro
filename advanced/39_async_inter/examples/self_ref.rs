#[derive(Debug)]
struct SelfReference {
    name: String,
    // 在初始化之后指向 name
    name_ptr: *const String,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        SelfReference {
            name: name.into(),
            name_ptr: std::ptr::null(),
        }
    }

    pub fn init(&mut self) {
        self.name_ptr = &self.name as *const String;
    }

    pub fn print_name(&self) {
        println!(
            "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
            self,
            &self.name,
            self.name_ptr,
            self.name,
            // 在使用 ptr 是需要 unsafe
            // SAFETY: 这里 name_ptr 潜在不安全，会指向旧的位置
            unsafe { &*self.name_ptr }
        )
    }
}

fn move_create_issue() -> SelfReference {
    let mut data = SelfReference::new("Test");
    data.init();

    data.print_name();

    let data = move_it(data);

    data.print_name();
    data
}

fn move_it(data: SelfReference) -> SelfReference {
    data
}

fn mem_swap_creates_issue() {
    let mut data1 = SelfReference::new("Test1");
    data1.init();
    let mut data2 = SelfReference::new("Test2");
    data2.init();
    data1.print_name();
    data2.print_name();
    std::mem::swap(&mut data1, &mut data2);
    data1.print_name();
    data2.print_name();
}

fn main() {
  let data = move_create_issue();
  println!("data: {:?}", data);
  // data.print_name();
  print!("\n");
  mem_swap_creates_issue();
}
