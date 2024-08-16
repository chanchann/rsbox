## workspace

在 Rust 项目中，`workspace` 定义了一个工作空间，它允许你在一个项目中管理多个相关的包（crates）。这个配置的作用是：

1. 组织项目结构：它定义了这个项目包含哪些子项目或模块。

2. 共享依赖：工作空间中的所有成员可以共享同一个 `Cargo.lock` 文件，确保所有成员使用相同版本的依赖。

3. 优化构建：Cargo 可以更智能地处理工作空间中各个成员之间的依赖关系，优化构建过程。

4. 便于管理：你可以在工作空间的根目录中运行 Cargo 命令，这些命令会应用到所有成员。

在这个特定的配置中，工作空间包括以下成员：
- `binance`（可能是主要的 Binance 相关代码）
- `binance/spot`（可能是 Binance 现货交易相关的代码）
- `binance/usdt`（可能是 Binance USDT 合约相关的代码）
- `logger`（日志记录模块）
- `pyalgo`（可能是一些 Python 算法的 Rust 实现或接口）

这种结构使得项目更加模块化，便于管理和开发大型 Rust 项目。

## 详解

ref : https://course.rs/cargo/reference/workspaces.html

https://kaisery.github.io/trpl-zh-cn/ch14-03-cargo-workspaces.html


随着项目开发的深入，库 crate 持续增大，而你希望将其进一步拆分成多个库 crate。Cargo 提供了一个叫 工作空间（workspaces）的功能，它可以帮助我们管理多个相关的协同开发的包。

### 工作空间的两种类型

root package 和虚拟清单( virtual manifest )

#### 根 package


若一个 package 的 Cargo.toml 包含了[package] 的同时又包含了 [workspace] 部分，则该 package 被称为工作空间的根 package。

```rust
[workspace]
members = [
  "crates/globset",
  "crates/grep",
  "crates/cli",
  "crates/matcher",
  "crates/pcre2",
  "crates/printer",
  "crates/regex",
  "crates/searcher",
  "crates/ignore",
]
```

## 虚拟清单

若一个 Cargo.toml 有 [workspace] 但是没有 [package] 部分，则它是虚拟清单类型的工作空间。

对于没有主 package 的场景或你希望将所有的 package 组织在单独的目录中时，这种方式就非常适合。

例如 rust-analyzer 就是这样的项目，它的根目录中的 Cargo.toml 中并没有 [package]，说明该根目录不是一个 package，但是却有 [workspace] :

```
[workspace]
members = ["xtask/", "lib/*", "crates/*"]
exclude = ["crates/proc_macro_test/imp"]
```

### 关键特性

所有的 package 共享同一个 Cargo.lock 文件，该文件位于工作空间的根目录中

所有的 package 共享同一个输出目录，该目录默认的名称是 target ，位于工作空间根目录下

只有工作空间根目录的 Cargo.toml 才能包含 [patch], [replace] 和 [profile.*]，而成员的 Cargo.toml 中的相应部分将被自动忽略


### example

我们的工作空间有一个二进制项目和两个库。二进制项目会提供主要功能，并会依赖另两个库。


```
$ mkdir add
$ cd add
touch Cargo.toml
```

```
// Cargo.toml
[workspace]

members = [
    "adder",
]
```

```
$ cargo new adder
     Created binary (application) `adder` package
```


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

## 在工作空间中依赖外部包

还需注意的是工作空间只在根目录有一个 Cargo.lock，而不是在每一个 crate 目录都有 Cargo.lock。这确保了所有的 crate 都使用完全相同版本的依赖。如果在 Cargo.toml 和 add_one/Cargo.toml 中都增加 rand crate，则 Cargo 会将其都解析为同一版本并记录到唯一的 Cargo.lock 中。使得工作空间中的所有 crate 都使用相同的依赖意味着其中的 crate 都是相互兼容的。

```
// add_one/Cargo.toml
[dependencies]
rand = "0.8.5"
```

现在就可以在 add_one/src/lib.rs 中增加 use rand; 了，接着在 add 目录运行 cargo build 构建整个工作空间就会引入并编译 rand crate：

```
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
   --snip--
   Compiling rand v0.8.5
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
warning: unused import: `rand`
 --> add_one/src/lib.rs:1:5
  |
1 | use rand;
  |     ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `add_one` (lib) generated 1 warning
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished dev [unoptimized + debuginfo] target(s) in 10.18s
```

现在顶级的 Cargo.lock 包含了 add_one 的 rand 依赖的信息。
然而，即使 rand 被用于工作空间的某处，也不能在其他 crate 中使用它，除非也在它们的 Cargo.toml 中加入 rand。
例如，如果在顶级的 adder crate 的 adder/src/main.rs 中增加 use rand;，会得到一个错误：

```
$ cargo build
  --snip--
   Compiling adder v0.1.0 (file:///projects/add/adder)
error[E0432]: unresolved import `rand`
 --> adder/src/main.rs:2:5
  |
2 | use rand;
  |     ^^^^ no external crate `rand`
```

为了修复这个错误，修改顶级 adder crate 的 Cargo.toml 来表明 rand 也是这个 crate 的依赖。构建 adder crate 会将 rand 加入到 Cargo.lock 中 adder 的依赖列表中，但是这并不会下载 rand 的额外拷贝。Cargo 确保了工作空间中任何使用 rand 的 crate 都采用相同的版本，这节省了空间并确保了工作空间中的 crate 将是相互兼容的。


### 为工作空间增加测试

```
// add_one/src/lib.rs

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
```

```
$ cargo test
   Compiling add_one v0.1.0 (file:///projects/add/add_one)
   Compiling adder v0.1.0 (file:///projects/add/adder)
    Finished test [unoptimized + debuginfo] target(s) in 0.27s
     Running unittests src/lib.rs (target/debug/deps/add_one-f0253159197f7841)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/adder-49979ff40686fa8e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```
$ cargo test -p add_one
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/add_one-b3235fea9a156f74)

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests add_one

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```
