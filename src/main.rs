use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt, AsyncWriteExt},
};

const BUFFER_SIZE: usize = 8192;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
    }
}

async fn router(mut stream: TcpStream) {
    let mut buf = [0u8; BUFFER_SIZE];

    loop {
        let n = match stream.read(&mut buf).await {
            Ok(0) => return,
            Ok(n) => n,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
        };
        if let Ok(s) = std::str::from_utf8(&buf[..n]) {
            println!("Received: {}", s);
        } else {
            println!("Received (non-UTF8): {:?}", &buf[..n]);
        }

        if let Err(e) = stream.write_all(&buf[0..n]).await {
            eprintln!("failed to write to socket; err = {:?}", e);
            return;
        }
    }

}
