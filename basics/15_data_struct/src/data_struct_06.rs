use std::{fmt, ops::Deref, str};
use std::borrow::{Cow};
use std::mem::{size_of};

use url::form_urlencoded::Target;

const MINI_STRING_MAX_LEN: usize = 30;

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    fn new(v: impl AsRef<str>) -> Self {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();
        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[..len].copy_from_slice(bytes);
        Self {
            len: len as u8,
            data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

impl MyString {
    fn push_str(&mut self, string: &str) {
        match self {
            MyString::Inline(m) => {
                let l = m.len();
                let len = l + string.len();
                if len > MINI_STRING_MAX_LEN {
                    *self = Self::Standard(m.to_string() + string);
                } else {
                    m.data[l..].copy_from_slice(string.as_bytes());
                    m.len = len as u8;
                }
            },
            MyString::Standard(m) => {
                m.push_str(string)
            }
        }
    }
}

impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match *self {
            MyString::Inline(ref v) => v.deref(),
            MyString::Standard(ref v) => v.deref(),
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        match s.len() > MINI_STRING_MAX_LEN {
            true => Self::Standard(s.to_owned()),
            _ => Self::Inline(MiniString::new(s)),
        }
    }
}

impl From<String> for MyString {
  fn from(s: String) -> Self {
      match s.len() > MINI_STRING_MAX_LEN {
          true => Self::Standard(s),
          _ => Self::Inline(MiniString::new(s)),
      }
  }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString {}", len1, len2);
    let s1: MyString = "hello world".into();
    let mut s2: MyString = "这是一个超过了三十个字节的很长很长的字符串".into();
    // debug 输出
    println!("s1: {:?}, s2: {:?}", s1, s2);
    s2.push_str("xxxxx");
    // display 输出
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );
    // MyString 可以使用一切 &str 接口，感谢 Rust 的自动 Deref
    assert!(s1.ends_with("world"));
    assert!(s2.starts_with("这"));

    println!("Cow<[u8]> {:?}", size_of::<Cow<[u8]>>());
    println!("Cow<str> {:?}", size_of::<Cow<str>>());
}
