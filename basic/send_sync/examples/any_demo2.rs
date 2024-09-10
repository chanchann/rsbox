/*
error[E0310]: the parameter type `T` may not live long enough
  --> any_demo2.rs:11:19
   |
11 |             data: Box::new(value),
   |                   ^^^^^^^^^^^^^^^
   |                   |
   |                   the parameter type `T` must be valid for the static lifetime...
   |                   ...so that the type `T` will meet its required lifetime bounds
   |
help: consider adding an explicit lifetime bound
   |
9  |     fn new<T: 'static>(value: T) -> Self {
   |             +++++++++

error[E0310]: the parameter type `T` may not live long enough
  --> any_demo2.rs:16:19
   |
16 |         self.data.downcast_ref::<T>()
   |                   ^^^^^^^^^^^^
   |                   |
   |                   the parameter type `T` must be valid for the static lifetime...
   |                   ...so that the type `T` will meet its required lifetime bounds
   |
help: consider adding an explicit lifetime bound
   |
15 |     fn get<T: 'static>(&self) -> Option<&T> {
   |             +++++++++

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0310`.
*/

/*
在 `Any` 的例子中使用 `'static` 生命周期是有特定原因的。让我解释一下：

1. `'static` 的含义：
   - `'static` 表示这个类型的生命周期可以持续到整个程序结束。
   - 它意味着这个类型不包含任何非静态引用。

2. 为什么 `Any` 需要 `'static`：
   - `Any` trait 主要用于运行时类型识别和转换。
   - 为了安全地进行这些操作，Rust 需要确保类型信息在整个程序运行期间都有效。
   - 只有 `'static` 生命周期的类型才能保证这一点。

3. 实际影响：
   - 这限制了 `Any` 只能用于不包含非静态引用的类型。
   - 这防止了一些潜在的内存安全问题，比如尝试访问已经失效的引用。

让我们通过一个例子来说明为什么需要 `'static`：

```rust
use std::any::Any;

struct Container<'a> {
    data: &'a str,
}

fn main() {
    let string = String::from("Hello");
    let container = Container { data: &string };
    
    // 下面这行会编译错误
    // let any_container: Box<dyn Any> = Box::new(container);
    
    // 这是因为 Container 包含一个非静态引用，
    // 所以它不能安全地转换为 Any 类型
}
```

在这个例子中，`Container` 包含一个非静态引用，因此它不能被安全地转换为 `Any` 类型。如果允许这样做，可能会导致在 `Any` 对象仍然存在时，它引用的数据已经被释放的情况。

相比之下，如果我们使用 `'static` 生命周期：

```rust
use std::any::Any;

struct StaticContainer {
    data: &'static str,
}

fn main() {
    let container = StaticContainer { data: "Hello" };
    
    // 这行可以编译
    let any_container: Box<dyn Any> = Box::new(container);
}
```

这个版本是可以编译的，因为 `StaticContainer` 只包含 `'static` 引用，所以它可以安全地转换为 `Any` 类型。

总结来说，`'static` 约束确保了通过 `Any` trait 进行类型操作的安全性，防止了可能的内存安全问题。这在处理未知类型时特别重要，比如在插件系统或泛型容器中。
*/

use std::any::Any;

// 定义一个可以存储任何类型的容器
struct AnyContainer {
    data: Box<dyn Any>,
}

impl AnyContainer {
    fn new<T>(value: T) -> Self {
        AnyContainer {
            data: Box::new(value),
        }
    }

    fn get<T>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }
}

fn main() {
    let container = AnyContainer::new(42);
    
    if let Some(value) = container.get::<i32>() {
        println!("值是: {}", value);
    }
    
    if let None = container.get::<String>() {
        println!("不是字符串类型");
    }
}
