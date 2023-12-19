use tokio_stream::StreamExt;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let mut stream = 
        tokio_stream::iter(["Lets", "Get", "Rusty"])
        .map(|s| s.to_ascii_uppercase());

    while let Some(s) = stream.next().await {
        println!("{s}");
    }

    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    // Write some data.
    stream.write_all(b"hello world!\n").await.unwrap();
}
