use std::time::Duration;
use tokio::time::interval;
use zmq::{Context, Error, Socket};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let context = Context::new();
    let sender = context.socket(zmq::PUSH)?;
    sender.connect("tcp://localhost:5555")?;

    let mut interval = interval(Duration::from_secs(3));
    let mut counter = 0;

    loop {
        interval.tick().await;
        counter += 1;
        let message = format!("Message {}", counter);
        
        send_with_retry(&sender, &message, 3).await?;
        println!("Sent: {}", message);
    }
}

async fn send_with_retry(socket: &Socket, message: &str, max_retries: usize) -> Result<(), Error> {
    for attempt in 0..max_retries {
        match socket.send(message, 0) {
            Ok(_) => return Ok(()),
            Err(e) if attempt == max_retries - 1 => return Err(e),
            Err(_) => {
                println!("Send failed, retrying...");
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }
    Ok(())
}