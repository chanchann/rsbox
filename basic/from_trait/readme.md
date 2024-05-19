## ref

https://rustwiki.org/zh-CN/rust-by-example/conversion/from_into.html

## From trait

From trait 允许一种类型定义 “怎么根据另一种类型生成自己”，因此它提供了一种类型转换的简单机制。在标准库中有无数 From 的实现，规定原生类型及其他常见类型的转换功能。

```rust
#![allow(unused)]
fn main() {
    let my_str = "hello";
    let my_string = String::from(my_str);
}
```

```rust
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}
```