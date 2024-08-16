use zmq;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 准备上下文和套接字
    let context = zmq::Context::new();
    let socket = context.socket(zmq::REP)?;
    socket.bind("tcp://*:5555")?;

    println!("服务器启动，等待客户端连接...");

    loop {
        // 0表示阻塞接收。如果没有消息可用，函数将会等待直到有消息到达。
        // 如果想使用非阻塞接收，可以使用zmq::DONTWAIT标志。
        let request = socket.recv_msg(0)?;
        println!("收到 {}", request.as_str().unwrap_or("未知消息"));

        // 做一些"处理"
        thread::sleep(Duration::from_secs(1));

        // 应答World
        socket.send("World", 0)?;
    }
}