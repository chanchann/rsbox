use tokio::runtime::*;
use tokio::time::*;
use chrono::Utc;

async fn job(i: i32) -> i32 {
    sleep(Duration::from_secs(2)).await;
    println!("job {}", i);
    i
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let handler = tokio::runtime::Handle::current();
    let h1 = handler.spawn(job(111));
    let h2 = handler.spawn(job(222));
    let h3 = handler.spawn(job(333));

    let start_time = Utc::now().time();

    let r1 = h1.await.unwrap();
    let r2 = h2.await.unwrap();
    let r3 = h3.await.unwrap();

    let end_time = Utc::now().time();
    println!("Cost: {}", (end_time - start_time).num_seconds());
}
