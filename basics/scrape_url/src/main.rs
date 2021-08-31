use std::fs;

fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
    f(value)
}

fn square(value: i32) -> i32 {
    value * value
}

fn cube(value: i32) -> i32 {
    value * value * value
}

fn pi() -> f64 {
    3.1415926
}

fn not_pi() {
    3.1415926;
}

#[derive(Debug)]
enum Gender {
    Unspecified = 0,
    Famale = 1,
    Male = 2,
}

#[derive(Debug, Clone, Copy)]
struct UserId(u64);
#[derive(Debug, Clone, Copy)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}
#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId, TopicId)),
    Leave((UserId, TopicId)),
    Message((UserId, TopicId, String)),
}


fn pub_calc(mut a: i32, mut b: i32) -> (i32, i32) {
    let c = a + b;
    a = b;
    b = c;
    println!("next val is {}", b);
    (a, b)
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    loop {
        let c = pub_calc(a, b);
        a = c.0;
        b = c.1;
        i += 1;

        if i >= n {
            break;
        }
    }
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = pub_calc(a, b);
        a = c.0;
        b = c.1;
        i += 1;
    }
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);
    for _i in 2..n {
        let c = pub_calc(a, b);
        a = c.0;
        b = c.1;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("apply square: {}", apply(2, square));
    println!("apply cube: {}", apply(2, cube));

    let is_pi = pi();
    let is_unit1 = not_pi();
    let is_unit2 = { pi(); };
    println!("is_pi: {:?}, is_unit1: {:?}, is_unit2: {:?}", is_pi, is_unit1, is_unit2);

    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Famale
    };
    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };
    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner: UserId(1),
    };

    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));
    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);

    let n = 10;
    fib_loop(n);
    fib_while(n);
    fib_for(n);

    fn process_event(event: &Event) {
        if let &Event::Message((_, _, msg)) = &event {
            println!("broadcast: {}", msg);
        }
    }

    process_event(&event3);

    let url = std::env::args().nth(1).unwrap();
    let output = "rust.md";
    println!("url is {}", url);

    let body = reqwest::blocking::get(url)?.text()?;
    fs::write(output, html2md::parse_html(&body).as_bytes())?;

    Ok(())
}
