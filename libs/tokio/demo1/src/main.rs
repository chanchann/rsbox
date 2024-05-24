use tokio::runtime::*;
use tokio::time::*;  // 不能使用std里的sleep
use chrono::Utc;

async fn job(i: i32) -> i32 {
    sleep(Duration::from_secs(2)).await;
    println!("job {}", i);
    i
}

fn main() {
    // 单线程
    // let runtime = Builder::new_current_thread();
    // 多线程
    let runtime = Builder::new_multi_thread().worker_threads(2).enable_time().build().unwrap();
    runtime.block_on(job(123));

    let start_time = Utc::now().time();

    runtime.block_on(async {
        let r1 = job(111).await;
        let r2 = job(222).await;
        let r3 = job(333).await;
        // 这里耗时 2 * 3 = 6s
        println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    });
    let end_time = Utc::now().time();
    println!("Cost: {}", (end_time - start_time).num_seconds());

    // 我们并行处理
    let h1 = runtime.spawn(job(111));
    let h2 = runtime.spawn(job(222));
    let h3 = runtime.spawn(job(333));

    let start_time = Utc::now().time();
    runtime.block_on(async {
        let r1 = h1.await.unwrap();
        let r2 = h2.await.unwrap();
        let r3 = h3.await.unwrap();
        println!("r1: {}, r2: {}, r3: {}", r1, r2, r3);
    });

    let end_time = Utc::now().time();
    // 2s
    println!("Cost: {}", (end_time - start_time).num_seconds());


}
