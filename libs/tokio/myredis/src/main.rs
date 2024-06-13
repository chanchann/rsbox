// 安装mini redis
// cargo install mini-redis
// run: mini-redis-server
//

use mini_redis::{client, Result};

/*
use mini_redis::Result;
use mini_redis::client::Client;
use tokio::net::ToSocketAddrs;

pub async fn connect<T: ToSocketAddrs>(addr: T) -> Result<Client> {
    // ...
}

async fn 的定义看起来像一个普通的同步函数，但却以异步方式运行。Rust在编译时将 async fn 转化为一个异步运行的routine。
在 async fn 中对 .await 的任何调用都会将控制权交还给线程。当操作在后台进行时，线程可以做其他工作。
*/

#[tokio::main]
async fn main() -> Result<()> {
    // Open a connection to the mini-redis address.
    // 它异步地与指定的远程地址建立了一个TCP连接。一旦连接建立起来，就会返回一个 client 句柄。尽管操作是异步进行的，但我们写的代码看起来是同步的。唯一表明该操作是异步的是 .await 操作符。
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set the key "hello" with value "world"
    client.set("hello", "world".into()).await?;

    // Get key "hello"
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
