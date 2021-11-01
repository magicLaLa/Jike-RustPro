use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use regex::Regex;
use glob::glob;

fn main() {
    let input_str = env::args().nth(1).unwrap_or("".to_string());
    let file_path = env::args().nth(2).unwrap_or("".to_string());
    let reg_str = Regex::new(&input_str).unwrap();
    if let Some(paths) = find_files(&file_path) {
        for path_buf in paths {
            if let Ok(p) = path_buf {
                if let Ok(lines) = read_lines(p) {
                    // 使用迭代器，返回一个（可选）字符串
                    for (i, line) in lines.enumerate() {
                        if let Ok(str) = line {
                            match reg_str.find(&str) {
                                Some(v) => println!("{}:{}  {}", i, v.start(), str),
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
    }
}

fn find_files(vague_path: &str) -> Option<glob::Paths> {
    let file_paths = glob(vague_path);
    match file_paths {
        Ok(p) => Some(p),
        _ => None
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
