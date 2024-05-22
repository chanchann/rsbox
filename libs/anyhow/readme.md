## 简介

REF: https://juejin.cn/post/7272015112818393148

anyhow是一个Rust库，用于简化错误处理和提供更好的错误报告。这个库适合用于应用程序，而不是用于创建库，因为它提供了一个非结构化的，方便使用的错误类型。

Rust的标准库提供了Result和Option类型用于错误处理，但它们通常需要指定错误类型。与此不同，anyhow::Result允许更简单地创建和处理错误。


## Result

```rust
// 使用标准库的 Result
fn do_something() -> Result<(), std::io::Error> {
    //...
    Ok(())
}

// 使用 anyhow::Result
fn do_something_anyhow() -> anyhow::Result<()> {
    //...
    Ok(())
}
```

anyhow::Result可以包含任何实现了std::error::Error的类型，可以更容易地与其他库互操作。

## 创建错误

```rust
use anyhow::anyhow;

let err = anyhow!("Something went wrong");
```

## 从其他错误类型转换

```rust
use anyhow::Error;

fn from_io_error(io_err: std::io::Error) -> Error {
    io_err.into()
}
```

## 链接错误

使用context和with_context可以为错误添加更多信息。

```rust
use anyhow::{Context, Result};

fn read_file(path: &str) -> Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("Failed to read file at {}", path))
}
```

## 显示和处理错误

### 使用?操作符

```rust
fn main() -> anyhow::Result<()> {
    let content = read_file("path/to/file")?;
    println!("{}", content);
    Ok(())
}
```

## 使用downcast检查特定错误类型

```rust
if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
    // Handle std::io::Error
}
```
