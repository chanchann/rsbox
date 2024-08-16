// 4. 在函数中使用可变借用：

fn add_one(num: &mut i32) {
    *num += 1;
}

fn main() {
    let mut x = 5;
    add_one(&mut x);
    println!("x is now {}", x);  // 输出：x is now 6
}