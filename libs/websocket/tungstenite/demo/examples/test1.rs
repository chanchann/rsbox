// simple server
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::stream::StreamExt;
use futures_util::sink::SinkExt;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8765".to_string();
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    println!("WebSocket server started on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_connection(stream));
    }
}

async fn handle_connection(stream: tokio::net::TcpStream) {
    let ws_stream = accept_async(stream).await.expect("Error during the websocket handshake");

    println!("New WebSocket connection");

    let (mut write, mut read) = ws_stream.split();

    while let Some(message) = read.next().await {
        match message {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    println!("Received message: {}", msg);
                    write.send(Message::Text(format!("Echo: {}", msg))).await.expect("Failed to send message");
                }
            }
            Err(e) => {
                println!("Error processing message: {}", e);
                break;
            }
        }
    }
}