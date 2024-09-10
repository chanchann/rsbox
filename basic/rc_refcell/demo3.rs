/*
线程安全的版本。我们将使用`Arc`（Atomic Reference Counted）代替`Rc`，并使用`Mutex`代替`RefCell`来确保线程安全。以下是修改后的代码：

这个线程安全版本的主要变化：

1. 将`Rc`替换为`Arc`：
   `Arc`提供了原子级的引用计数，可以安全地在线程间共享。

2. 将`RefCell`替换为`Mutex`：
   `Mutex`提供了互斥访问，确保在任何时候只有一个线程可以访问数据。

3. 使用`lock()`方法访问数据：
   每次访问`Arc<Mutex<T>>`中的数据时，我们需要调用`lock()`方法并处理可能的错误。

4. 创建多个线程：
   在`main`函数中，我们创建了两个线程来演示并发访问。

5. 克隆`Arc`以在线程间共享：
   我们克隆`Arc`指针以在不同的线程中使用。

这个版本的代码是线程安全的，可以在多线程环境中使用。但请注意，使用`Mutex`可能会带来一些性能开销，并且如果不小心使用，可能会导致死锁。在实际应用中，您可能需要根据具体的并发需求来调整这个结构。

另外，这种循环引用的设计仍然可能导致内存泄漏，因为`Arc`不能处理循环引用。在实际应用中，您可能需要考虑使用`Weak`指针来打破循环引用，或者重新设计数据结构以避免循环引用。
*/

use std::sync::{Arc, Mutex};
use std::thread;

struct A {
    data: String,
    b: Option<Arc<Mutex<B>>>,
}

struct B {
    a: Arc<Mutex<A>>,
    reference_to_a_data: String,
}

impl A {
    fn new(data: String) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(A {
            data,
            b: None,
        }))
    }

    fn set_b(&mut self, b: Arc<Mutex<B>>) {
        self.b = Some(b);
    }
}

impl B {
    fn new(a: Arc<Mutex<A>>) -> Arc<Mutex<Self>> {
        let reference_to_a_data = a.lock().unwrap().data.clone();
        let b = Arc::new(Mutex::new(B {
            a: a.clone(),
            reference_to_a_data,
        }));
        a.lock().unwrap().set_b(b.clone());
        b
    }
}

fn main() {
    let a = A::new(String::from("示例数据"));
    let b = B::new(a.clone());
    
    let a_clone = a.clone();
    let b_clone = b.clone();

    let thread1 = thread::spawn(move || {
        let a_data = a_clone.lock().unwrap().data.clone();
        println!("线程1 - A中的数据: {}", a_data);
    });

    let thread2 = thread::spawn(move || {
        let b_data = b_clone.lock().unwrap().reference_to_a_data.clone();
        println!("线程2 - B引用的A的数据: {}", b_data);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();

    println!("主线程 - A中的数据: {}", a.lock().unwrap().data);
    println!("主线程 - B引用的A的数据: {}", b.lock().unwrap().reference_to_a_data);
}