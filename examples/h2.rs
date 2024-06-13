use std::error::Error;
use std::time::Duration;

use bytes::Bytes;
use h2::server::{self, SendResponse};
use h2::RecvStream;
use http::Request;
use tokio::net::{TcpListener, TcpStream};

use h2::client;
use http::HeaderMap;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let server = server();
    let client = client();
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

async fn server() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:5928").await?;

    let timeout_duration = Duration::from_secs(3);
    println!("server: listening on {:?}", listener.local_addr());

    loop {
        match timeout(timeout_duration, listener.accept()).await {
            Ok(Ok((socket, _peer_addr))) => {
                tokio::spawn(async move {
                    if let Err(e) = serve(socket).await {
                        println!("server:  -> err={:?}", e);
                    }
                });
            }
            Ok(Err(e)) => {
                println!("server: accept error {:?}", e);
                break;
            }
            Err(_) => {
                println!("server: timeout reached, shutting down");
                break;
            }
        }
    }

    Ok(())
}

async fn serve(socket: TcpStream) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut connection = server::handshake(socket).await?;
    println!("server: H2 connection bound");

    while let Some(result) = connection.accept().await {
        let (request, respond) = result?;
        tokio::spawn(async move {
            if let Err(e) = handle_request(request, respond).await {
                println!("server: error while handling request: {}", e);
            }
        });
    }

    println!("server: ~~~~~~~~~~~ H2 connection CLOSE !!!!!! ~~~~~~~~~~~");
    Ok(())
}

async fn handle_request(
    mut request: Request<RecvStream>,
    mut respond: SendResponse<Bytes>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    println!("server: GOT request: {:?}", request);

    let body = request.body_mut();
    while let Some(data) = body.data().await {
        let data = data?;
        println!("server: <<<< recv {:?}", data);
        let _ = body.flow_control().release_capacity(data.len());
    }

    let response = http::Response::new(());
    let mut send = respond.send_response(response, false)?;
    println!("server: >>>> send");
    send.send_data(Bytes::from_static(b"hello "), false)?;
    send.send_data(Bytes::from_static(b"world\n"), true)?;

    Ok(())
}

pub async fn client() -> Result<(), Box<dyn Error>> {
    let tcp = TcpStream::connect("127.0.0.1:5928").await?;
    let (mut client, h2) = client::handshake(tcp).await?;

    println!("client: sending request");

    let request = Request::builder()
        .uri("https://http2.akamai.com/")
        .body(())
        .unwrap();

    let mut trailers = HeaderMap::new();
    trailers.insert("zomg", "hello".parse().unwrap());

    let (response, mut stream) = client.send_request(request, false).unwrap();

    // send trailers
    stream.send_trailers(trailers).unwrap();

    // Spawn a task to run the conn...
    tokio::spawn(async move {
        if let Err(e) = h2.await {
            println!("client: GOT ERR={:?}", e);
        }
    });

    let response = response.await?;
    println!("client: GOT RESPONSE: {:?}", response);

    // Get the body
    let mut body = response.into_body();

    while let Some(chunk) = body.data().await {
        println!("client: GOT CHUNK = {:?}", chunk?);
    }

    if let Some(trailers) = body.trailers().await? {
        println!("client: GOT TRAILERS: {:?}", trailers);
    }
    println!("client: exit");
    Ok(())
}
