/*
`Rc`是"Reference Counted"的缩写，它是Rust标准库中提供的一种智能指针类型。以下是`Rc`的主要特点和用途：

1. 共享所有权：
   `Rc`允许多个所有者共享同一数据的所有权。当最后一个所有者被销毁时，数据才会被释放。

2. 引用计数：
   `Rc`内部维护一个引用计数。每当创建一个新的`Rc`指针（通过克隆），计数就会增加；当`Rc`指针离开作用域时，计数就会减少。

3. 不可变借用：
   `Rc`只允许不可变借用。如果需要可变访问，通常需要配合`RefCell`使用。

4. 单线程使用：
   `Rc`不是线程安全的。对于多线程场景，应使用`Arc`（Atomic Reference Counted）。

5. 创建和克隆：
   - 使用`Rc::new()`创建新的`Rc`
   - 使用`clone()`方法创建新的引用

6. 性能考虑：
   虽然`Rc`提供了灵活性，但它确实带来了一些运行时开销（引用计数的管理）。

`Rc`的常见用途：

1. 实现共享所有权的数据结构（如图或树）。
2. 在不同部分的代码中共享只读数据。
3. 实现某些设计模式，如观察者模式。

使用`Rc`时需要注意：

1. 避免创建循环引用，这可能导致内存泄漏。如果需要，可以使用`Weak`指针来打破循环。
2. 当需要可变访问时，考虑使用`Rc<RefCell<T>>`的组合。
3. 在单线程环境中使用`Rc`；多线程环境中使用`Arc`。

总的来说，`Rc`是Rust中处理共享所有权的重要工具，但应谨慎使用，并理解其性能影响和限制。
*/


use std::rc::Rc;

fn main() {
    // 创建一个Rc
    let data = Rc::new(5);

    // 克隆Rc，增加引用计数
    let data2 = data.clone();
    let data3 = Rc::clone(&data);

    println!("引用计数: {}", Rc::strong_count(&data)); // 输出: 3

    // 使用Rc中的数据
    println!("数据: {}", *data);

    // data, data2, data3 离开作用域时，引用计数减少
    // 当计数为0时，数据被释放
}

