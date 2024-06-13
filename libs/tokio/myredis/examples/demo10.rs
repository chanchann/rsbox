// 共享状态
// 使用 Mutex 来保护共享状态
// 我们有一个键值服务器在工作。然而，有一个重大的缺陷：状态没有在不同的连接中共享。我们将在这篇文章中解决这个问题。

// 在Tokio中，共享状态有几种不同的方式：
// 1. 用Mutex来保护共享状态。
// 2. 生成一个任务来管理状态，并使用消息传递来操作它。

// 对于简单的数据使用第一种方法
// 需要异步工作的东西使用第二种方法，比如I/O原语

// 这里共享状态是一个HashMap，操作是 insert 和 get。这两种操作都不是异步的，所以我们将使用Mutex。

// 添加 bytes 依赖
// Mini-Redis板块没有使用 Vec<u8>，而是使用 bytes crate的 Bytes。
// Bytes的目标是为网络编程提供一个强大的字节数组结构。
// 相比 Vec<u8> 最大的特点是浅层克隆。换句话说，在 Bytes 实例上调用 clone() 并不复制基础数据。
// 相反，Bytes 实例是对一些底层数据的一个引用计数的句柄。Bytes类型大致是一个Arc<Vec<u8>，但有一些附加功能。

use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;

// 注意，使用 std::sync::Mutex 而不是 tokio::Mutex 来保护 HashMap。
// 一个常见的错误是在异步代码中无条件地使用 tokio::sync::Mutex。异步Mutex是一个跨调用 .await 而被锁定的Mutex。
// 同步的mutex在等待获得锁的时候会阻塞当前线程。这反过来又会阻塞其他任务的处理。
// 然而，切换到 tokio::sync::Mutex 通常没有帮助，因为异步mutex内部使用同步mutex。
// 作为一个经验法则，在异步代码中使用同步的mutex是可以的，只要竞争保持在较低的水平，并且在调用 .await 时不保持锁。
// 此外，可以考虑使用parking_lot::Mutex 作为 std::sync::Mutex 的更快的替代品。

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    println!("Listening");

    // HashMap将在许多任务和可能的许多线程之间共享。为了支持这一点，它被包裹在 Arc<Mutex<_>> 中。
    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        // Clone the handle to the hash map.
        // 使用Arc允许从许多任务中并发地引用HashMap，可能在许多线程上运行
        let db = db.clone();

        println!("Accepted");
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}

