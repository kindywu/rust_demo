use futures::StreamExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}

async fn handle_connection(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // Create an mpsc channel to communicate between the reader and writer tasks
    let (tx, rx) = mpsc::channel(32);

    let mut rx = ReceiverStream::new(rx);

    let (mut reader, mut writer) = socket.into_split();

    // Spawn a task to read data from the socket and send it to the writer
    let reader_handle = tokio::spawn(async move {
        let mut buf = [0; 1024];

        loop {
            let n = match reader.read(&mut buf).await {
                Ok(n) if n == 0 => return, // Connection closed
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Failed to read from socket: {}", e);
                    return;
                }
            };
            if tx.send(buf[..n].to_vec()).await.is_err() {
                return; // Receiver dropped
            }
        }
    });

    // Spawn a task to write data to the socket
    let writer_handle = tokio::spawn(async move {
        while let Some(data) = rx.next().await {
            if let Err(e) = writer.write_all(&data).await {
                eprintln!("Failed to write to socket: {}", e);
                return;
            }
        }
    });

    // Wait for both tasks to complete
    let _ = tokio::try_join!(reader_handle, writer_handle);
    Ok(())
}
