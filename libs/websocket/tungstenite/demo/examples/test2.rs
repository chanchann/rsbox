use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use url::Url;

#[tokio::main]
async fn main() {
    let url = Url::parse("ws://127.0.0.1:8765").expect("Invalid URL");
    // let url = Url::parse("wss://echo.websocket.org").expect("Invalid URL");

    let (mut ws_stream, _) = connect_async(url.to_string()).await.expect("Failed to connect");

    println!("WebSocket client connected");

    // Send a message to the server
    let msg = Message::Text("Hello, WebSocket server!".to_string());
    ws_stream.send(msg).await.expect("Failed to send message");

    // Receive a message from the server
    if let Some(Ok(message)) = ws_stream.next().await {
        println!("Received message from server: {}", message);
    }

    // Close the connection
    ws_stream.close(None).await.expect("Failed to close connection");
}