use std::error::Error;
use std::sync::Arc;
use http::Request;
use log::{error, info};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use client::ProntoClient;
use crate::{AppState, BackendError};

// TODO: Should not be backend error result
#[tokio::main]
pub async fn run_proxy_thread(context: AppState) -> Result<(), BackendError> {
    loop {
        if context.is_loaded().await {
            break;
        }
    }
    let client = {
        let state = context.inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        state.client.clone()
    };

    let listen_addr = "127.0.0.1:10521";
    let server_addr = client.api_base_url.clone();

    info!("Proxy Server Started\nListening on: {}\nProxying to: {}", listen_addr,server_addr);

    let listener = TcpListener::bind(listen_addr).await?;

    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn({
            let client = client.clone();

            async move {
                if let Err(e) = process(&mut stream, client).await {
                    error!("failed to process connection; error = {}", e);
                }
            }
        });
    }
}

async fn parse_request(stream: &mut TcpStream) -> (String, String) {
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).await.unwrap();
    let request_str = std::str::from_utf8(&buffer).unwrap();

    let lines: Vec<String> = request_str.lines().map(|line| line.to_string()).collect();
    let request_line = lines.first().unwrap().to_string();

    // get body
    let mut collect = false;
    let mut body = String::from("");
    for line in &lines {
        if collect {
            body.push_str(line);
        }
        if line.is_empty() {
            collect = true;
        }
    }
    body = body.trim_matches(char::from(0)).to_string();

    (request_line, body)
}

async fn process(stream: &mut TcpStream, client: Arc<ProntoClient>) -> Result<(), Box<dyn Error>> {
    let parse_request = parse_request(stream).await;
    info!("{}", parse_request.0);
    let request_line = parse_request.0;
    let method = request_line.split_whitespace().collect::<Vec<&str>>()[0];
    let uri = request_line.split_whitespace().collect::<Vec<&str>>()[1];
    let request = Request::builder()
        .method(method)
        .uri(uri);

    // TODO: Headers

    let request = request
        .body(parse_request.1)
        .unwrap();
    // TODO: hardcoded
    let response = client.http_client.request(request.method().clone(), format!("https://files.chat.trypronto.com/{}", request.uri().clone().to_string()))
        .body(request.body().to_string())
        .send().await?;
    // send response
    let response = response.bytes().await?;
    stream.write_all(&response).await?;
    Ok(())
}
