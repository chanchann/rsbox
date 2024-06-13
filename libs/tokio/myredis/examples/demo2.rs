// 异步main函数
// 1. 它是 async fn
// 2. 它被注解为 #[tokio::main]

/*
使用 async fn 是因为我们想进入一个异步上下文
异步函数必须由一个运行时来执行。运行时包含异步任务调度器，提供事件化I/O、计时器等。运行时不会自动启动，所以主函数需要启动它

*/

#[tokio::main]
async fn main() {
    println!("hello");
}

/*
==
fn main() {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        println!("hello");
    })
}
*/

/*
tokio = { version = "1", features = ["full"] }

启用了 full 的功能标志:

Tokio 有很多功能（TCP、UDP、Unix 套接字、定时器、同步工具、多种调度器类型等）。不是所有的应用程序都需要所有的功能。
当试图优化编译时间或最终应用程序的足迹时，应用程序可以决定只选择进入它所使用的功能。
*/