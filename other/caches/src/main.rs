// https://matklad.github.io/2022/06/11/caches-in-rust.html
use std::{collections::HashMap, io};

#[derive(Debug)]
struct App<'a> {
    config: HashMap<&'a str, bool>,
    db: HashMap<&'a str, bool>,
    cache: HashMap<u32, Widget>,
}

#[derive(Debug, Clone)]
struct Widget {
    title: String,
}

impl App<'static> {
    pub fn load(&self, _key: &[u8]) -> io::Result<Option<u8>> {
        todo!()
    }

    pub fn get_widget(&mut self, id: u32) -> io::Result<Option<Widget>> {
        let key = id.to_be_bytes();
        if self.cache.contains_key(&id) {
            let widget = self.cache.get(&id).unwrap();
            return Ok(Some(widget.clone()));
        }
        let _value = match self.load(&key)? {
            Some(it) => it,
            None => return Ok(None),
        };
        let widget = Widget {
            title: "trst".to_string(),
        };
        self.cache.insert(id, widget);
        let widget = self.cache.get(&id).unwrap().clone();
        Ok(Some(widget))
    }
}

fn main() {
    let mut app = App {
        config: HashMap::new(),
        db: HashMap::new(),
        cache: HashMap::new(),
    };
    app.config.insert("a", false);
    println!("app: {:?}", app);
}
