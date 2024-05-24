use tokio::time::*;
use tokio::task;

#[tokio::main]
async fn main() {
    // 串行
    async {
        sleep(Duration::from_secs(3)).await;
        println!("hello");
    }.await;  // await这里要等待
    async {
        sleep(Duration::from_secs(3)).await;
        println!("111");
    }.await;
    println!("--------------------------world");


    // 并行
    let h1 = task::spawn(async {
        sleep(Duration::from_secs(3)).await;
        println!("222");
    });
    let h2 = task::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("333");
    });
    h1.await.unwrap();
    h2.await.unwrap();
    println!("---------------------------444");


    let h1 = task::spawn(async {
        sleep(Duration::from_secs(3)).await;
        "h1"
    });
    let h2 = task::spawn(async {
        sleep(Duration::from_secs(2)).await;
        "h2"
    });
    let (_, _) = tokio::join!(h1, h2);
    println!("---------------------------55555555");
}
