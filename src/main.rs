use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("localhost:8080").await?;
    
    loop {
        let (mut socket, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop {
                let n = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,

                    Err(e) => {
                        eprintln!("Failed to read from socket, Error = {:?}.", e);
                        return;
                    },
                };

                if let Err(e) = socket.write_all(&buffer[0 .. n]).await {
                    eprintln!("Failed to write to socket, Error = {:?}.", e);
                    return;
                }
            }
        });
    }
}
