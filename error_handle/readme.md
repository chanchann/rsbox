## ref

https://zhuanlan.zhihu.com/p/148604981

## 1. Option - 可空变量

虽然Rust中有null的概念，但是使用null并不是Rust中常见的模式

地道的Rust代码应该让该函数返回一个Option。Option或更确切的说Option<T>是一个泛型，可以是Some<T>或None

Rust将Some和None称为变体（Variant）

```rust
fn find_store(mobile_os: &str) -> Option<&str> {
    match mobile_os {
        "iOS" => Some("App Store"),
        "android" => Some("Play Store"),
        _ => None
    }
}

fn main() {
    println!("{}", match find_store("windows") {
        Some(s) => s,
        None => "Not a valid mobile OS"
    });
}
```

## 2. Result - 包含错误信息的结果

Result<T,E>，是和Rust中的Option相关的概念，它是一个加强版本的Option。

Ok(T)：结果为成员T

Err(E)：结果为故障成员E


```rust
// 解析JSON字符串的serde_json库
/*
pub fn from_str<'a, T>(s: &'a str) -> Result<T, Error> 
where
    T: Deserialize<'a>,
*/
let json_string = r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

let p:Person = match serde_json::from_str(json_string) {
    Ok(p) => p,
    Err(e) => ... //we will discuss what goes here next 
};
```

当碰到Err时，我们可以采取两个动作：

1. panic!

2. 返回Err

## 3. unwrap - 故障时执行panic！

```rust
let p: Person = match serde_json::from_str(data) {
        Ok(p) => p,
        Err(e) => panic!("cannot parse JSON {:?}, e"), //panic
    }

// ==
let p:Person = serde_json::from_str(data).unwrap();
```

如果我们可以确定输入的json_string始终会是可解析的，那么使用unwrap没有问题

```rust
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example() -> Result<()> {
    //age2 is error on purpose
    let data = r#"
        {
            "name": "John Doe",
            "age2": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let p:Person = serde_json::from_str(data).unwrap();

    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(())
}

fn main() {
    match typed_example() {
        Ok(_) => println!("program ran ok"),
        Err(_) => println!("program ran with error"),
    }
}
```

## 4 ? - 故障时返回Err对象

当碰到Err时，我们不一定要panic!，也可以返回Err。不是每个Err都是不可恢复的，因此有时并不需要panic!

```rust
let p: Person = match serde_json::from_str(data) {
        Ok(p) => p,
        Err(e) => return Err(e.into()),
};

// == 
let p:Person = serde_json::from_str(data)?;
```

# 5 使用unwrap和?解包Option

```rust
fn next_birthday(current_age: Option<u8>) -> Option<String> {
	// If `current_age` is `None`, this returns `None`.
	// If `current_age` is `Some`, the inner `u8` gets assigned to `next_age` after 1 is added to it
    let next_age: u8 = current_age?;
    Some(format!("Next year I will be {}", next_age + 1))
}

fn main() {
  let s = next_birthday(None);
  match s {
      Some(a) => println!("{:#?}", a),
      None => println!("No next birthday")
  }
}
```
