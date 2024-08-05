// 5. 在结构体中使用可变借用：

struct Counter {
    count: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.count += 1;
    }
}


fn main() {
    let mut counter = Counter { count: 0 };
    counter.increment();
    println!("Count is now {}", counter.count);  // 输出：Count is now 1
}
