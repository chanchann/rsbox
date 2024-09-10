/*
Any 允许在运行时进行类型检查和转换。
Send 允许将数据安全地发送到其他线程。
Sync 允许多个线程同时访问同一数据。
*/

use std::any::Any;
use std::sync::Arc;
use std::thread;

trait Message: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Debug)]
struct TextMessage {
    content: String,
}

impl Message for TextMessage {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn process_message(msg: Arc<dyn Message>) {
    thread::spawn(move || {
        if let Some(text_msg) = msg.as_any().downcast_ref::<TextMessage>() {
            println!("处理文本消息: {:?}", text_msg);
        } else {
            println!("未知消息类型");
        }
    }).join().unwrap();
}

fn main() {
    let msg = Arc::new(TextMessage {
        content: "Hello, World!".to_string(),
    });
    
    process_message(msg);
}
