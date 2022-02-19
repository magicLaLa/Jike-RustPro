use anyhow::Result;
use futures::{io::sink, prelude::*};
use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<()> {
    let file_sink = write(File::create("/tmp/hello").await?);
    futures::pin_mut!(file_sink);
    if let Err(_) = file_sink.send("hello\\n").await {
        println!("Error on send");
    }
    if let Err(_) = file_sink.send("world\\n").await {
        println!("Error on send");
    }
    Ok(())
}

/// 使用 unflod 生成一个 Sink 的结构
fn write<'a>(file: File) -> impl Sink<&'a str> {
    sink::unfold(file, |mut file, line: &'a str| async move {
        file.write_all(line.as_bytes()).await?;
        eprint!("input: {}", line);
        Ok::<_, std::io::Error>(file)
    })
}
