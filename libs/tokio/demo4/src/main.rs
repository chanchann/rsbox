// tokio 有两大线程
// 1. 工作线程worker thread，用于异步任务
// 2. 阻塞线程 blocking thread，用于同步任务


use tokio::time::*;
use tokio::task;


#[tokio::main(flavor="current_thread")]
async fn main() {
    let h1 = task::spawn(async {
        sleep(Duration::from_secs(4)).await;   
        println!("1111");
        "h1"
    });
    let h2 = task::spawn(async {
        sleep(Duration::from_secs(3)).await;   
        println!("2222");
        "h2"
    });
    let h3 = task::spawn(async {
        sleep(Duration::from_secs(1)).await;   
        println!("3333");
        "h3"
    });

    // 这里默认是个512个线程的线程池。不会阻塞当前线程
    let h4 = task::spawn_blocking(|| {
        // 注意这里是标准库的sleep, 如果是上面写法，会阻塞当前线程
        std::thread::sleep(Duration::from_secs(2)).await;
        println!("4444");
        "h4"
    })

    h1.await.unwrap();
    h2.await.unwrap();
    h3.await.unwrap();

    println!("-------------");
}
