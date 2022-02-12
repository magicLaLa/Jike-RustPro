use std::{marker::PhantomPinned, pin::Pin};

#[derive(Debug)]
struct SelfReference {
    name: String,
    // 在初始化之后指向 name
    name_ptr: *const String,
    // 占位符
    _marker: PhantomPinned,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        SelfReference {
            name: name.into(),
            name_ptr: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        let name_ptr = &self.name as *const String;
        let this = unsafe { self.get_unchecked_mut() };
        this.name_ptr = name_ptr;
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

fn move_create_issue() {
    let mut data = SelfReference::new("Test");
    let mut data = unsafe {
        Pin::new_unchecked(&mut data)
    };
    SelfReference::init(data.as_mut());

    data.as_ref().print_name();

    move_pinned(data.as_mut());
    println!("{:?} ({:p})", data, &data);

    // move_it(data)

}

fn move_pinned(data: Pin<&mut SelfReference>) {
    println!("{:?} ({:p})", data, &data);
}

#[allow(dead_code)]
fn move_it(data: SelfReference) {
    println!("{:?} ({:p})", data, &data);
}

fn main() {
    move_create_issue();
}
