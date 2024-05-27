use tokio::time::*;
use tokio::task;
#[tokio::main]
async fn main() {
    let t1 = tokio::time::timeout(Duration::from_secs(2), async {
        sleep(Duration::from_secs(5)).await; 
        println!("Task done");               
    });

    match t1.await {
        Ok(()) => {
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    let t2 = tokio::time::timeout(Duration::from_secs(2), async {
        sleep(Duration::from_secs(1)).await; 
        panic!("inner error");
    });
    let h2 = task::spawn(t2);
    // 这里看是内部还是外部错误
    h2.await.expect("inner error").expect("timeout error");
}
