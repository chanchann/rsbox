// 任务
// tokio::spawn 函数返回 JoinHandle，调用者可以用它来与生成的任务进行交互。
// 该异步块可以有一个返回值。调用者可以使用 JoinHandle 上的 .await 获取返回值。

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        // Do some async work
        "return value"
    });

    // Do some other work

    let out = handle.await.unwrap();
    println!("GOT {}", out);
}
