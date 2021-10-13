
use std::collections::BTreeMap;

#[derive(Debug, Eq)]
struct Name {
    pub name: String,
    pub flags: u32,
}

impl Name {
    pub fn new(name: impl AsRef<str>, flags: u32) -> Self {
        Self {
            name: name.as_ref().to_string(),
            flags,
        }
    }
}

impl PartialOrd for Name {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
      Some(self.cmp(other))
  }
}

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        (self.flags, &self.name) == (other.flags, &other.name)
    }
}

impl Ord for Name {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      (self.flags, &self.name).cmp(&(other.flags, &other.name))
  }
}

fn main() {
    let mut map = BTreeMap::new();
    map.insert(Name::new("/etc/password", 0x1), 12);
    map.insert(Name::new("/etc/hosts", 0x1), 4);
    map.insert(Name::new("/home/tchen", 0x0), 28);

    for item in map.iter() {
        println!("{:?}", item);
    }
}
