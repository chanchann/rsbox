/*
if let是Rust中一种简洁的模式匹配方式，主要用于只关心一种匹配情况时，可以避免使用完整的match表达式。

*/

fn main() {
    // 示例1：处理Option类型
    let some_value: Option<i32> = Some(42);
    if let Some(x) = some_value {
        println!("值是: {}", x);
    } else {
        println!("没有值");
    }

    // 示例2：处理Result类型
    let result: Result<i32, &str> = Ok(10);
    if let Ok(value) = result {
        println!("成功: {}", value);
    } else {
        println!("错误");
    }

    // 示例3：匹配枚举
    enum Color {
        Red,
        Green,
        Blue(u8),
    }
    let color = Color::Blue(255);
    if let Color::Blue(value) = color {
        println!("是蓝色，亮度为: {}", value);
    } else {
        println!("不是蓝色");
    }

    // 示例4：结构体匹配
    struct Point {
        x: i32,
        y: i32,
    }
    let point = Point { x: 0, y: 7 };
    if let Point { x: 0, y } = point {
        println!("在y轴上，y = {}", y);
    } else {
        println!("不在y轴上");
    }

    // 示例5：带条件的if let
    let number = Some(7);
    if let Some(x) = number {
        if x > 5 {
            println!("大于5的数字: {}", x);
        }
    }
}