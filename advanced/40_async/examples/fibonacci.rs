use futures::{prelude::*, stream::poll_fn};
use std::{task::Poll};

#[tokio::main]
async fn main() {
  consume(fib().take(10)).await;
  consume((fib1(10))).await;
  consume((fib2(10).boxed())).await;
}

async fn consume(mut st: impl Stream<Item = i32> + Unpin) {
  while let Some(v) = st.next().await {
      print!("{}", v);
  }
  println!("\n");
}

// 使用 repeat_with 创建 stream，无法控制何时结束
fn fib() -> impl Stream<Item = i32> {
  let mut a = 1;
  let mut b = 1;
  stream::repeat_with(move || {
    let c = a + b;
    a = b;
    b = c;
    b
  })
}

// 使用 poll_fn 创建 stream，可以通过返回 Poll::Ready(None) 来结束
fn fib1(mut n: usize) -> impl Stream<Item = i32> {
  let mut a = 1;
  let mut b = 1;
  poll_fn(move |_cx| -> Poll<Option<i32>> {
    if n == 0 {
      return Poll::Ready(None);
    }
    n -= 1;
    let c = a + b;
    a = b;
    b = c;
    Poll::Ready(Some(b))
  })
}

fn fib2(n: usize) -> impl Stream<Item = i32> {
  stream::unfold((n, (1, 1)), |(mut n, (a, b))| async move {
    if n == 0 {
      None
    } else {
      n -= 1;
      let c = a + b;
      Some((c, (n, (b , c))))
    }
  })
}