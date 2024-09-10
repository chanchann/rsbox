use std::thread;

// 实现 Send 的结构体
#[derive(Debug)]
struct SendableData {
    value: i32,
}

// 默认情况下，包含原始类型的结构体是 Send 的
impl SendableData {}

fn main() {
    let data = SendableData { value: 42 };
    
    let handle = thread::spawn(move || {
        println!("在新线程中: {:?}", data);
    });
    
    handle.join().unwrap();
}
