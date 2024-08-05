// 3. 可变借用的生命周期

fn main() {
    let mut x = 5;
    {
        let y = &mut x;  // y 的生命周期开始
        *y += 1;
    }  // y 的生命周期结束
    println!("x is now {}", x);  // 正确：y 的生命周期已经结束
}