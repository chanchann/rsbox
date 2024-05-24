use tokio::task;
use tokio::sync::mpsc;
use tokio::time::*;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(5);

    task::spawn(async move {
        let mut i = 0;
        while i < 10 {
            println!("Send {} ..., cap: {}", i, tx.capacity());
            tx.send(i).await.unwrap();
            i += 1;
        }
    });

    sleep(Duration::from_secs(3)).await;
    
    while let Some(ret) = rx.recv().await {
        if ret == 7 {
            rx.close();
        }
        println!("recv : {}", ret);
    }
}
