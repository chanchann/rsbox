```
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

工作空间在顶级目录有一个 target 目录；adder 并没有自己的 target 目录。即使进入 adder 目录运行 cargo build，构建结果也位于 add/target 而不是 add/adder/target。工作空间中的 crate 之间相互依赖。如果每个 crate 有其自己的 target 目录，为了在自己的 target 目录中生成构建结果，工作空间中的每一个 crate 都不得不相互重新编译其他 crate。通过共享一个 target 目录，工作空间可以避免其他 crate 重复构建。

```
[workspace]

members = [
    "adder",
    "add_one",
]
```

```
cargo new add_one --lib
```

```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

```
// add_one/src/lib.rs
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

二进制 adder 依赖库 crate add_one。首先需要在 adder/Cargo.toml 文件中增加 add_one 作为路径依赖：

```
// adder/Cargo.toml

[dependencies]
add_one = { path = "../add_one" }
```

cargo 并不假定工作空间中的 Crates 会相互依赖，所以需要明确表明工作空间中 crate 的依赖关系。

```
// adder/src/main.rs
use add_one;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", add_one::add_one(num));
}
```

在 add 目录中运行 cargo build 来构建工作空间！

```
$ cargo build
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 0.68s
```

为了在顶层 add 目录运行二进制 crate，可以通过 -p 参数和包名称来运行 cargo run 指定工作空间中我们希望使用的包：

```
$ cargo run -p adder
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
     Running `target/debug/adder`
Hello, world! 10 plus one is 11!
```