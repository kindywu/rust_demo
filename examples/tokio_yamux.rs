use anyhow::Result;
use futures::prelude::*;
use log::{info, warn};
use tokio::net::{TcpListener, TcpStream};
use tokio_util::codec::{Framed, LinesCodec};
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
            while let Some(Ok(stream)) = session.next().await {
                let stream_id = stream.id();
                info!(
                    "Server accept a stream from client: stream_id = {}",
                    stream_id
                );

                let mut frame = Framed::new(stream, LinesCodec::new());

                tokio::spawn(async move {
                    loop {
                        match frame.next().await {
                            Some(Ok(message)) => {
                                info!("[server] read data: {}", message);

                                info!("[server] send '{}' to remote", message);
                                if let Err(e) = frame.send(message).await {
                                    warn!("[server] read LinesCodecError: {}", e);
                                    break;
                                }
                            }
                            Some(Err(e)) => {
                                warn!("[server] LinesCodecError: {}", e);
                                break;
                            }
                            None => break,
                        }
                    }
                });

                info!("[server] stream_id {stream_id} exit");
            }
        });
    }

    Ok(())
}

async fn run_client() -> Result<()> {
    let socket = TcpStream::connect("127.0.0.1:12345").await.unwrap();
    info!("[client] connected to server: {:?}", socket.peer_addr());
    let mut session = Session::new_client(socket, Config::default());
    let stream = session.open_stream().unwrap();

    let mut frame = Framed::new(stream, LinesCodec::new());

    tokio::spawn(async move {
        loop {
            match session.next().await {
                Some(Ok(_)) => (),
                Some(Err(e)) => {
                    info!("{}", e);
                    break;
                }
                None => {
                    info!("[client] closed");
                    break;
                }
            }
        }
    });

    for i in 1..=3 {
        let message = format!("hello world {}", i);
        info!("[client] send '{message}' to remote");
        frame.send(message).await.unwrap();

        info!("[client] reading data");
        let message = frame.next().await.unwrap().unwrap();
        info!("[client] read data: {:?}", message);
    }

    Ok(())
}
