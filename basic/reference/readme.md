在Rust中，`&`符号主要用于创建引用。引用是Rust中非常重要的概念，它们允许你引用某些值而不获取其所有权。以下是一些使用`&`的常见情况：

1. 创建不可变引用：

```rust
let x = 5;
let y = &x; // y是x的不可变引用
```

2. 函数参数：

当你想在函数中使用一个值但不想获取其所有权时，可以使用引用作为参数。

```rust
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

let my_string = String::from("Hello");
print_length(&my_string);
```

3. 方法调用：

许多方法接受引用作为参数，而不是获取值的所有权。

```rust
let v = vec![1, 2, 3];
v.iter().for_each(|&x| println!("{}", x));
```

4. 模式匹配：

在模式匹配中，可以使用`&`来匹配引用。

```rust
match &some_value {
    &Some(x) => println!("Got a value: {}", x),
    &None => println!("No value"),
}
```

5. 创建可变引用：

使用`&mut`可以创建可变引用。

```rust
let mut x = 5;
let y = &mut x; // y是x的可变引用
*y += 1;
```

6. 解引用：

虽然这不是直接使用`&`，但值得一提的是`*`操作符用于解引用。

```rust
let x = 5;
let y = &x;
assert_eq!(5, *y);
```

7. 字符串字面量：

字符串字面量默认就是`&str`类型，这是一个字符串切片引用。
```rust
let s: &str = "Hello, world!";
```

8. 集合的借用：

当你想遍历集合但不想获取其所有权时。

```rust
let v = vec![1, 2, 3];
for i in &v {
    println!("{}", i);
}
```

9. 实现trait：

当为类型实现trait时，通常会使用`&self`或`&mut self`作为方法的第一个参数。

```rust
impl MyTrait for MyStruct {
    fn do_something(&self) {
        // ...
    }
}
```

记住，使用引用可以避免不必要的复制和移动，提高性能，并允许多个部分的代码同时访问相同的数据（遵循借用规则）。