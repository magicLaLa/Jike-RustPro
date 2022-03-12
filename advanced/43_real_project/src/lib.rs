//! 这是 creat 文档

use std::task::Poll;
use futures::{prelude::*, stream::poll_fn};
use std::io;
use std::fs;

/// fibnacci 算法
/// 示例：
/// ```
/// use futures::prelude::*;
/// use playground::fib; // playground crate 名字叫 playground
/// # futures::executor::block_on(async {
/// let mut st = fib(10);
/// assert_eq!(Some(2), st.next().await);
/// # });
/// ```
pub fn fib(mut n: usize) -> impl Stream<Item = i32> {
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

/// 写入文件
/// 示例：
/// ```
/// use playground::write_file;
/// write_file("/tmp/dummy_test", "hello world")?;
/// # Ok::<_, std::io::Error>(())
/// ```
pub fn write_file(name: &str, contents: &str) -> Result<(), io::Error> {
    fs::write(name, contents)
}