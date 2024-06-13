// 可见看见demo6中的报错提示增加个move
// 默认情况下，变量不会被 move 到异步块中。v向量仍然归主函数所有
// task::spawn(async move { 将指示编译器将v move 到被催生的任务中。这样，该任务拥有其所有的数据，使其成为 ‘static。
// 如果数据必须被多个任务同时访问，那么它必须使用同步原语（如Arc）进行共享。

use tokio::task;

#[tokio::main]
async fn main() {
    let v = vec![1, 2, 3];

    task::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });
}

