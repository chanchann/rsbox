// 1. 在结构体中使用生命周期：

struct MyStruct<'a> {
    x: &'a i32,
}

fn main() {
    let value = 5;
    let my_struct = MyStruct { x: &value };
    println!("Value: {}", my_struct.x);
}