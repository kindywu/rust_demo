use std::time::Duration;

use anyhow::Result;
use futures::{SinkExt, StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    time::sleep,
};

use tokio_util::codec::{Framed, LinesCodec};
use tokio_yamux::{session::SessionType, Config, Session};

#[tokio::main]
async fn main() -> Result<()> {
    let client = client();
    let server = server();

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

async fn client() -> Result<()> {
    println!("start client");

    // 创建一个 TCP 连接
    let stream = TcpStream::connect("127.0.0.1:8080").await?;

    // 创建一个 yamux 会话配置
    let config = Config::default();

    // 使用配置创建一个 yamux 会话
    let mut session = Session::new(stream, config, SessionType::Client);

    // 创建一个新的子流
    let substream = session.open_stream()?;

    sleep(Duration::from_secs(3)).await;

    let mut substream = Framed::new(substream, LinesCodec::new());

    // 向子流写入数据
    if let Err(e) = substream.send("Hello, world!").await {
        eprintln!("client failed to write to substream: {}", e);
    }
    println!("client send");

    // 从子流读取数据
    match substream.next().await {
        Some(Ok(line)) => println!("client received: {}", line),
        Some(Err(e)) => eprintln!("client failed to read from substream: {}", e),
        None => eprintln!("client failed to read from substream"),
    }
    println!("client exit");
    // 关闭子流
    Ok(())
}

async fn server() -> Result<()> {
    println!("start server");
    // 创建一个 TCP 监听器
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // 创建一个 yamux 会话配置
    let config = Config::default();

    loop {
        let (stream, addr) = listener.accept().await?;
        let session = Session::new(stream, config, SessionType::Server);
        println!("server accept client {}", addr);
        // 为每个会话创建一个任务
        tokio::spawn(async move {
            if let Err(e) = handle_session(session).await {
                eprintln!("server session error: {}", e);
            }
        });
    }
}

async fn handle_session(mut session: Session<tokio::net::TcpStream>) -> Result<()> {
    println!("server handle session");

    loop {
        match session.next().await {
            Some(Ok(stream)) => {
                // 处理子流
                println!("server handle sub stream");
                let stream = Framed::new(stream, LinesCodec::new());

                let (mut writer, mut reader) = stream.split();

                println!("server split stream");
                tokio::spawn(async move {
                    loop {
                        let line = match reader.next().await {
                            Some(Ok(line)) => line,
                            Some(Err(e)) => {
                                eprintln!("{e}");
                                break;
                            }
                            None => {
                                eprintln!("none");
                                break;
                            }
                        };

                        println!("server receive {line}");
                        if let Err(e) = writer.send(line).await {
                            println!("server failed to send line {}", e);
                        }
                    }
                });
            }
            Some(Err(e)) => {
                eprintln!("server failed to accept stream: {}", e);
                break;
            }
            None => {
                eprintln!("server failed to accept stream");
                break;
            }
        }
    }

    Ok(())
}
