use std::fs::File;
use std::io::Read;

fn read_file(name: &str) -> Result<String, std::io::Error> {
    let mut f = File::open(name)?;
    let mut  contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    if let Ok(lall) = read_file("../Cargo.toml").and_then(|s| {
        Ok(s)
    }) {
        println!("lall {:#?}", lall);
    };
}
