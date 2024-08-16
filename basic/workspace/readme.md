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





















