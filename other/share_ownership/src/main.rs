// use crossbeam::scope;
// use std::{sync::Arc, thread::spawn};

// #[derive(Debug)]
// struct User {
//     name: String,
// }

// fn main() {
//     let user_original = Arc::new(User {
//         name: "drogus".to_string(),
//     });

//     let user = user_original.clone();
//     let t1 = spawn(move || {
//         println!("Hello from the first thread {}", user.name);
//     });

//     let user = user_original.clone();
//     let t2 = spawn(move || {
//         println!("Hello from the second thread {}", user.name);
//     });

//     t1.join().unwrap();
//     t2.join().unwrap();
// }

// // fn main() {
// //     let user = User { name: "drogus".to_string() };

// //     scope(|s| {
// //         s.spawn(|_| {
// //             println!("Hello from first thread {}", &user.name);
// //         });

// //         s.spawn(|_| {
// //             println!("Hello from first thread {}", &user.name);
// //         });
// //     }).unwrap();

// // }

// use std::{
//     sync::{Arc, Mutex},
//     thread::{self, spawn},
//     time::Duration,
// };

// #[derive(Debug)]
// struct User {
//     name: String,
// }

// fn main() {
//     let user_original = Arc::new(Mutex::new(User {
//         name: "original".to_string(),
//     }));

//     let user = user_original.clone();
//     let t1 = spawn(move || {
//         let mut locked_user = user.lock().unwrap();
//         locked_user.name = String::from("piotr");
//     });

//     let user = user_original.clone();
//     let t2 = spawn(move || {
//         thread::sleep(Duration::from_millis(10));
//         println!("Hello {}", user.lock().unwrap().name);
//     });

//     t1.join().unwrap();
//     t2.join().unwrap();
// }

use std::{sync::Mutex, thread::sleep, time::Duration};

use crossbeam::scope;

struct User {
    name: String,
}

fn main() {
    let user = Mutex::new(User {
        name: "drogus".to_string(),
    });

    scope(|s| {
        s.spawn(|_| {
            user.lock().unwrap().name = String::from("piotr");
        });

        s.spawn(|_| {
            sleep(Duration::from_secs(10));
            println!("Hello {}", user.lock().unwrap().name);
        });
    })
    .unwrap();
}
