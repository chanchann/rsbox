// 很类似go的chennel的消息传递方式
// 某异步任务负责发消息，其他异步任务收消息

// 几种类型
// oneshot : 一对一发送的一次性通道，一个发送者(Sender)发送最多一个数据，只有一个接收者
// mpsc: 多对一发送的通道，多个发送者，一个接收
// broadcast : 多对多
// watch ： 一对多

use tokio::task;
use tokio::sync::oneshot;
use tokio::time::*;

#[tokio::main]
async fn main() {  
    let (tx, rx) = oneshot::channel::<i32>();
    let sender = task::spawn(async {
        sleep(Duration::from_secs(2)).await;
        tx.send(123).unwrap();
    });

    let reciever = task::spawn(async {
        let ret = rx.await.unwrap();
        println!("reciever : {}", ret);
    });

    reciever.await.unwrap();
}
