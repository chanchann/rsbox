/*
error[E0502]: cannot borrow `data` as mutable because it is also borrowed as immutable
  --> mutable_borrow.rs:7:9
   |
3  |     let data1 = vec![&data[0]];
   |                       ---- immutable borrow occurs here
...
7  |         data.push(i);
   |         ^^^^^^^^^^^^ mutable borrow occurs here
...
11 |     println!("boxed: {:p}", &data1);
   |                             ------ immutable borrow later used here

error: aborting due to 1 previous error
*/

fn main() {
    let mut data = vec![1, 2, 3];
    let data1 = vec![&data[0]];
    println!("data[0]: {:p}", &data[0]);

    for i in 0..100 {
        data.push(i);
    }

    println!("data[0]: {:p}", &data[0]);
    println!("boxed: {:p}", &data1);
}

/*
补充一些知识点

在 Rust 中，可变借用（mutable borrow) 允许你临时获得对一个值的可变引用，以便修改该值。以下是关于可变借用的一些关键点：

1. 排他性：在同一时间，只能有一个可变借用存在。这意味着当存在可变借用时，不能同时存在其他的可变借用或不可变借用。

2. 生命周期：可变借用的生命周期不能超过被借用的值的生命周期。

3. 使用方式：通过 `&mut` 语法创建可变借用。

4. 借用规则：
   - 在任意给定时间，要么只能有一个可变引用，要么只能有任意数量的不可变引用。
   - 引用必须总是有效的。

5. 安全性：可变借用确保了内存安全和数据竞争的避免。

在你提供的代码中，`data` 首先被不可变地借用（`let data1 = vec![&data[0]];`），然后尝试进行可变借用（`data.push(i);`）。

这违反了借用规则，因为不可变借用仍然有效（在最后的 println! 中使用），所以不能同时进行可变借用。

这种机制帮助 Rust 在编译时就能捕获潜在的并发问题和数据竞争，提高了程序的安全性和可靠性。

*/