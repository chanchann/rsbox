/* 
RefCell是Rust标准库中的一个类型，它提供了"内部可变性"的机制。以下是RefCell的主要特点和用途：

1. 内部可变性：
   RefCell允许你在不可变引用的情况下修改数据。这在Rust的借用规则下通常是不允许的，但RefCell在运行时进行借用检查，使得这成为可能。

2. 运行时借用检查：
   与编译时的借用检查不同，RefCell在运行时执行借用规则。这意味着如果违反了借用规则（例如同时存在多个可变引用），程序会在运行时panic。

3. 使用方法：
   - `borrow()`: 创建一个不可变引用
   - `borrow_mut()`: 创建一个可变引用

4. 性能考虑：
   由于借用检查发生在运行时，使用RefCell会有一些性能开销。

5. 线程安全性：
   RefCell不是线程安全的。如果需要在多线程环境中使用，应考虑使用Mutex或RwLock。

6. 常见用途：
   - 在不可变结构中修改特定字段
   - 实现内部可变性模式
   - 在某些情况下绕过编译器的借用检查

在使用RefCell时需要注意：

1. 过度使用可能导致代码难以理解和维护。
2. 如果可能，优先考虑使用Rust的常规借用规则。
3. 要小心避免运行时的借用错误，这可能导致程序panic。

总的来说，RefCell是Rust中处理特殊情况的有力工具，但应谨慎使用。
*/

use std::cell::RefCell;

let data = RefCell::new(5);

{
    let mut mut_borrow = data.borrow_mut();
    *mut_borrow += 1;
}

{
    let borrow = data.borrow();
    println!("数据是: {}", *borrow);
}

