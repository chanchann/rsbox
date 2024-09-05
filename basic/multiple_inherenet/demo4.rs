// 默认 trait 方法
// 你可以在 trait 中提供默认实现,这样实现该 trait 的类型就可以选择性地覆盖这些方法:

trait Trait1 {
    fn method1(&self) {
        println!("Default Method 1");
    }
}

trait Trait2 {
    fn method2(&self) {
        println!("Default Method 2");
    }
}

struct MyStruct;

impl Trait1 for MyStruct {}
impl Trait2 for MyStruct {}

// MyStruct 现在有了 method1 和 method2 的默认实现

fn main() {
    let my_struct = MyStruct;
    my_struct.method1();
    my_struct.method2();
}