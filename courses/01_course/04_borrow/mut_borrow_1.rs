// 1. 基本的可变借用：

fn main() {
    let mut x = 5;
    let y = &mut x;  // 可变借用
    *y += 1;  // 通过可变借用修改值
    println!("x is now {}", x);  // 输出：x is now 6
}