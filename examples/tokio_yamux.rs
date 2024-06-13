use anyhow::Result;
use futures::prelude::*;
use log::info;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tokio_yamux::{config::Config, session::Session};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let client = run_client();
    let server = run_server();

    let res = tokio::try_join!(server, client);
    match res {
        Ok((_, _)) => {
            println!("ok");
        }
        Err(err) => {
            eprintln!("processing failed; error = {}", err);
        }
    }
    Ok(())
}

async fn run_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();

    while let Ok((socket, _)) = listener.accept().await {
        info!("accepted a socket: {:?}", socket.peer_addr());
        let mut session = Session::new_server(socket, Config::default());
        tokio::spawn(async move {
            while let Some(Ok(mut stream)) = session.next().await {
                info!("Server accept a stream from client: id={}", stream.id());
                tokio::spawn(async move {
                    let mut data = [0u8; 3];
                    stream.read_exact(&mut data).await.unwrap();
                    info!("[server] read data: {:?}", data);

                    info!("[server] send 'def' to remote");
                    stream.write_all(b"def").await.unwrap();

                    let mut data = [0u8; 2];
                    stream.read_exact(&mut data).await.unwrap();
                    info!("[server] read again: {:?}", data);
                });
            }
        });
    }

    Ok(())
}

async fn run_client() -> Result<()> {
    let socket = TcpStream::connect("127.0.0.1:12345").await.unwrap();
    info!("[client] connected to server: {:?}", socket.peer_addr());
    let mut session = Session::new_client(socket, Config::default());
    let mut stream = session.open_stream().unwrap();

    tokio::spawn(async move {
        loop {
            match session.next().await {
                Some(Ok(_)) => (),
                Some(Err(e)) => {
                    info!("{}", e);
                    break;
                }
                None => {
                    info!("closed");
                    break;
                }
            }
        }
    });

    info!("[client] send 'abc' to remote");
    stream.write_all(b"abc").await.unwrap();

    info!("[client] reading data");
    let mut data = [0u8; 3];
    stream.read_exact(&mut data).await.unwrap();
    info!("[client] read data: {:?}", data);

    info!("[client] send 'dd' to remote");
    stream.write_all(b"dd").await.unwrap();
    stream.shutdown().await.unwrap();
    Ok(())
}
