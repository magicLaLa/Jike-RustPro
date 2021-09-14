
// fn main() {
//     let s1 = String::from("Lindsey");
//     let s2 = String::from("Rosie");

//     let result = max(&s1, &s2);

//     println!("bigger one: {}", result);

//     let res = get_max(&s1);
//     println!("res is: {}", res);
// }

// fn get_max<'a>(s1: &'a str) -> &'a str {
//     max(s1, "cynthia")
// }

// fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
//     if s1 > s2 {
//         s1
//     } else {
//         s2
//     }
// }

struct Employee<'a, 'b> {
    name: &'a str,
    title: &'b str,
    age: u8,
}

pub fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    if let Some(i) = s.find(delimiter) {
        let prefix = &s[..i];
        // 由于 delimiter 可以是 utf8，所以我们需要获得其 utf8 长度，
        // 直接使用 len 返回的是字节长度，会有问题
        let suffix = &s[(i + delimiter.len_utf8())..];
        *s= suffix;
        prefix
    } else {
        let prefix = *s;
        *s = "";
        prefix
    }
}

fn main() {
    let s = "hello world".to_owned();
    let mut s1 = s.as_str();
    let hello = strtok(&mut s1, ' ');
    println!("hello is: {}, s1: {}, s: {}", hello, s1, s);
}