## ref

https://zhuanlan.zhihu.com/p/351354764

## basic

default trait 可以用于定义类型的默认值。基础用法举例：

```rust
#[derive(Debug)]
struct TraitTest {
    enable: bool,
    type_id: i32,
}
impl Default for TraitTest {
    fn default() -> Self {
        TraitTest {
            enable: true,
            type_id: 1,
        }
    }
}

fn main() {
    let td = TraitTest::default();

    println!("{:?}", td);
}
```

如果我们就是想用字段类型本身的默认值，不想像上面的例子一样指定特定值的话，用上面的写法明显会有大段的冗余代码

```
#[derive(Default, Debug)]
struct TraitTest2 {
    a: i32,
    b: bool,
}

fn main() {
    let td2 = TraitTest2::default();
    println!("{:?}", td2);
}
```

## 只想指定类型其中一个或几个字段的值

```rust
fn main() {
    let td3 = TraitTest {
        enable: false,
        ..TraitTest::default()
    };

    println!("{:?}", td3);
}
```

## Simpler ways

```rust
#![feature(default_free_fn)]
use std::default::default;


/// ... def code

fn main() {
    let td3 = TraitTest {
        enable: false,
        ..default()
    };

    println!("{:?}", td3);
}
```