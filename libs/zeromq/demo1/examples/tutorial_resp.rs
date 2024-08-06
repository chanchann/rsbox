use zmq;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = zmq::Context::new();
    
    println!("正在连接至hello world服务端...");
    let requester = context.socket(zmq::REQ)?;
    requester.connect("tcp://localhost:5555")?;

    for request_nbr in 0..10 {
        print!("正在发送 Hello {}...", request_nbr);
        // io::stdout().flush()? 是用于立即刷新标准输出缓冲区的操作。
        // 在Rust中，输出通常是行缓冲的，意味着输出会在遇到换行符时自动刷新。
        // 但是，当你使用 print! 宏（而不是 println!）时，输出可能会留在缓冲区中，不会立即显示。
        // flush() 方法强制将缓冲区中的所有数据立即写入标准输出，
        // 确保 "正在发送 Hello ..." 消息立即显示在屏幕上，即使它后面没有换行符。
        io::stdout().flush()?;
        requester.send("Hello", 0)?;

        let reply = requester.recv_string(0)?;
        println!("接收到 {} {}", reply.unwrap(), request_nbr);
    }

    Ok(())
}