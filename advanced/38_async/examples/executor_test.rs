use futures::executor::block_on;
use std::future::Future;

#[tokio::main]
async fn main() {
  let name1 = "tesst".to_string();
  let name2 = "world".to_string();

  say_hello1(&name1).await;
  say_hello2(&name2).await;

  // Future 除了可以用 await 来执行外 还可以用 executor 执行

  block_on(say_hello1(&name1));
  block_on(say_hello2(&name2));
}

async fn say_hello1(name: &str) -> usize {
  println!("hrllo {}", name);
  42
}

// async fn 关键字相当于一个返回 impl Future 的语法糖
fn say_hello2<'a>(name: &'a str) -> impl Future<Output = usize> + 'a {
  async move {
    println!("hello {}", name);
    42
  }
}