// 2. 在函数参数中使用生命周期：

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("short");
    let s2 = String::from("longer");
    let result = longest(&s1, &s2);
    println!("Longest string: {}", result);
}

// 3. 多个生命周期参数：

fn multiple_lifetimes<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}