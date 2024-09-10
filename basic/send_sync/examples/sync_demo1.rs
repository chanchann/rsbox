use std::sync::Arc;
use std::thread;

// 实现 Sync 的结构体
struct SyncData {
    value: i32,
}

// 默认情况下，只包含实现了 Sync 的类型的结构体也是 Sync 的
impl SyncData {}

fn main() {
    let data = Arc::new(SyncData { value: 42 });
    
    let mut handles = vec![];
    
    for _ in 0..5 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("在线程中读取值: {}", data_clone.value);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
