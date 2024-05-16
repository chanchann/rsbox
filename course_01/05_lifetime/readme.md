## ref

https://www.zhihu.com/question/435470652

## why need?

必须确保依赖有效

定义值的有效期：开始于其自身的原作用域之开始，结束于其依赖项的作用域结束。

```rust
/*0*/{
/*1*/    let r;
/*2*/
/*3*/    {
/*4*/        let x = 5;
/*5*/        r = &x;
/*6*/    }
/*7*/
/*8*/    println!("r: {}", r);
/*9*/}
```

r 的（原本）作用域：1~9 行。

依赖的作用域：4~5 行。

r 的真实有效期为 1~5 行。故第 8 行引用了无效的 r，因此编译报错。

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
```

若 a 依赖 b，则 b 的有效期应当大于 a

## 函数参数与返回值间的依赖

```rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}


fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

longest 的返回值（记作 a），a 依赖 x, y，而 x, y 的有效期未知，导致 r 的有效期未知，故不安全。

解决方法：标注返回值的生命周期 'a ，到 x, y 上：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

从而表明返回值 a 的依赖项 x, y 具有相同有效期（或者说，x, y 的有效期大于等于 a）。从而通过检查。

```rust
/*1*/fn main() {
/*2*/    let string1 = String::from("long string is long");
/*3*/    let result;
/*4*/    {
/*5*/        let string2 = String::from("xyz");
/*6*/        result = longest(string1.as_str(), string2./**/as_str());
/*7*/    }
/*8*/    println!("The longest string is {}", result);
/*9*/}
```

为何不能编译?

result 实际 3-6, 因为他依赖string2，string2 到6失效

8已经失效了

## 结构体与其成员的依赖

结构由成员构成，则结构依赖于成员，则成员有效期必须大于结构有效期。

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

