// 2. 可变借用的排他性：

fn main() {
    let mut x = 5;
    let y = &mut x;  // 第一个可变借用
    // let z = &mut x;  // 错误：不能同时存在两个可变借用
    // let w = &x;  // 错误：不能同时存在可变借用和不可变借用
    *y += 1;
    println!("x is now {}", x);
}