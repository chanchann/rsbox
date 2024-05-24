use tokio::task;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(10);
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    task::spawn(async move {
        let mut i = 0;
        while i < 10 {
            tx1.send(i).await.unwrap();
            i += 1;
        }
    });

    task::spawn(async move {
        let mut i = 0;
        while i < 10 {
            tx2.send(i).await.unwrap();
            i += 1;
        }
    });
    drop(tx);  // important
    while let Some(ret) = rx.recv().await {
        println!("recv : {}", ret);
    }
}
